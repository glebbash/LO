#![no_std]
#![feature(alloc_error_handler, thread_local)]

extern crate alloc;
mod ast;
mod codegen;
mod common;
mod io;
mod lexer;
mod parser;
mod printer;
mod registry;
mod typer;
mod wasm;
mod wasm_eval;

use crate::{codegen::*, common::*, printer::*, registry::*, typer::*, wasm_eval::*};

static USAGE: &str = "Usage:
  lo compile <input.lo>
  lo inspect <input.lo>
  lo format <input.lo>
  lo eval <input.lo> (experimental)";

#[unsafe(no_mangle)]
pub extern "C" fn _start() {
    let args = WasiArgs::load().unwrap();
    if args.size < 3 {
        stderr_writeln(USAGE);
        proc_exit(1);
    }

    let command = args.get(1).unwrap();
    let mut file_name = args.get(2).unwrap();
    if file_name == "-i" {
        file_name = "<stdin>";
    }

    let mut registry = Registry::new();

    // for debug purposes only, not public api
    if command == "lex" {
        registry.in_single_file_mode = true;
        registry.in_lex_only_mode = true;

        let Some(module) = registry
            .relax_mut()
            .include_file(file_name, &Loc::internal())
        else {
            proc_exit(1)
        };

        let file_info = &registry.fm.files[module.parser.lexer.file_index];
        stdout_writeln(format!("file_path: {}", file_info.absolute_path));

        stdout_enable_buffering();

        let source = module.parser.lexer.source;
        for token in &module.parser.lexer.tokens {
            stdout_writeln(format!(
                "{}:{}-{}:{} ({}-{}) {:?} >> {} <<",
                token.loc.pos.line,
                token.loc.pos.col,
                token.loc.end_pos.line,
                token.loc.end_pos.col,
                token.loc.pos.offset,
                token.loc.end_pos.offset,
                token.type_,
                token.loc.read_span(source).replace("\n", "\\n"),
            ));
        }

        stdout_disable_buffering();

        return;
    }

    if command == "format" {
        registry.in_single_file_mode = true;

        let Some(module) = registry.include_file(file_name, &Loc::internal()) else {
            proc_exit(1);
        };

        let mut printer = Printer::new(module.parser.be_mut());
        printer.print_file();

        return;
    }

    if command == "inspect" {
        registry.reporter.begin_inspection();
    }

    registry.include_file(file_name, &Loc::internal());

    let mut typer = Typer::new(&mut registry);
    typer.type_all();

    let mut codegen = CodeGenerator::new(&mut registry);
    codegen.codegen_all();

    registry.process_deferred_intrinsics();

    if registry.reporter.in_inspection_mode {
        registry.reporter.end_inspection();

        if *registry.reporter.error_count == 0 {
            return;
        }
    }

    if *registry.reporter.error_count > 0 {
        proc_exit(1);
    }

    if command == "compile" {
        let mut binary = Vec::new();
        codegen.wasm_module.dump(&mut binary);
        stdout_write(binary.as_slice());
        return;
    }

    if command == "eval" {
        let mut eval = WasmEval::new(&mut codegen.wasm_module);
        eval.wasi_args = Some(args);
        eval.wasi_args_skip = 2;

        catch!(eval.eval(), err, {
            stderr_writeln(err.message);
            proc_exit(1);
        });
        return;
    }

    stderr_writeln(format!("Unknown command: {command}\n{}", USAGE));
    proc_exit(1);
}

mod no_std_options {
    use super::lol_alloc::LolAlloc;

    #[global_allocator]
    static ALLOCATOR: LolAlloc = LolAlloc::new();

    #[alloc_error_handler]
    fn oom(_: core::alloc::Layout) -> ! {
        core::arch::wasm32::unreachable()
    }

    #[cfg(not(test))]
    #[panic_handler]
    fn panic(info: &core::panic::PanicInfo<'_>) -> ! {
        crate::io::stderr_write(alloc::format!("{info}\n"));
        core::arch::wasm32::unreachable();
    }
}

// modified version of https://github.com/Craig-Macomber/lol_alloc, MIT License
mod lol_alloc {
    use core::{
        alloc::{GlobalAlloc, Layout},
        cell::UnsafeCell,
        ptr::{self, null_mut},
    };

    /// The WebAssembly page size, in bytes.
    const PAGE_SIZE: usize = 65536;

    /// Invalid number of pages used to indicate out of memory errors.
    const ERROR_PAGE_COUNT: usize = usize::MAX;

    /// A non-thread safe allocator that uses a free list.
    /// Allocations and frees have runtime O(length of free list).
    ///
    /// The free list is kept sorted by address, and adjacent blocks of memory are coalesced when inserting new blocks.
    pub struct LolAlloc {
        free_list: UnsafeCell<*mut FreeListNode>,
    }

    impl LolAlloc {
        pub const fn new() -> Self {
            LolAlloc {
                // Use a special value for empty, which is never valid otherwise.
                free_list: UnsafeCell::new(EMPTY_FREE_LIST),
            }
        }
    }

    const EMPTY_FREE_LIST: *mut FreeListNode = usize::MAX as *mut FreeListNode;

    /// Stored at the beginning of each free segment.
    /// Note: It would be possible to fit this in 1 word (use the low bit to flag that case,
    /// then only use a second word if the allocation has size greater than 1 word)
    struct FreeListNode {
        next: *mut FreeListNode,
        size: usize,
    }

    const NODE_SIZE: usize = core::mem::size_of::<FreeListNode>();

    /// This is an invalid implementation of Sync.
    /// FreeListAllocator must not actually be used from multiple threads concurrently.
    unsafe impl Sync for LolAlloc {}

    unsafe impl GlobalAlloc for LolAlloc {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            unsafe {
                // This assumes PAGE_SIZE is always a multiple of the required alignment, which should be true for all practical use.
                debug_assert!(PAGE_SIZE % layout.align() == 0);

                let size = full_size(layout);
                let alignment = layout.align().max(NODE_SIZE);
                let mut free_list: *mut *mut FreeListNode = self.free_list.get();
                // search freelist
                loop {
                    if *free_list == EMPTY_FREE_LIST {
                        break;
                    }
                    // Try to allocate from end of block of free space.
                    let size_of_block = (**free_list).size;
                    let start_of_block = *free_list as usize;
                    let end_of_block = start_of_block + size_of_block;
                    if size < end_of_block {
                        let position = multiple_below(end_of_block - size, alignment);
                        if position >= start_of_block {
                            // Compute if we need a node after used space due to alignment.
                            let end_of_used = position + size;
                            if end_of_used < end_of_block {
                                // Insert new block
                                let new_block = end_of_used as *mut FreeListNode;
                                (*new_block).next = *free_list;
                                (*new_block).size = end_of_block - end_of_used;
                                *free_list = new_block;
                                free_list = ptr::addr_of_mut!((*new_block).next);
                            }
                            if position == start_of_block {
                                // Remove current node from free list.
                                *free_list = (**free_list).next;
                            } else {
                                // Shrink free block
                                (**free_list).size = position - start_of_block;
                            }

                            let ptr = position as *mut u8;
                            debug_assert!(ptr.align_offset(NODE_SIZE) == 0);
                            debug_assert!(ptr.align_offset(layout.align()) == 0);
                            return ptr;
                        }
                    }

                    free_list = ptr::addr_of_mut!((**free_list).next);
                }

                // Failed to find space in the free list.
                // So allocate more space, and allocate from that.
                // Simplest way to due that is grow the heap, and "free" the new space then recurse.
                // This should never need to recurse more than once.

                let requested_bytes = round_up(size, PAGE_SIZE);
                let previous_page_count =
                    core::arch::wasm32::memory_grow(0, requested_bytes / PAGE_SIZE);
                if previous_page_count == ERROR_PAGE_COUNT {
                    return null_mut();
                }

                let ptr = (previous_page_count * PAGE_SIZE) as *mut u8;
                self.dealloc(
                    ptr,
                    Layout::from_size_align_unchecked(requested_bytes, PAGE_SIZE),
                );
                self.alloc(layout)
            }
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            unsafe {
                debug_assert!(ptr.align_offset(NODE_SIZE) == 0);
                let ptr = ptr as *mut FreeListNode;
                let size = full_size(layout);
                let after_new = offset_bytes(ptr, size); // Used to merge with next node if adjacent.

                let mut free_list: *mut *mut FreeListNode = self.free_list.get();
                // Insert into freelist which is stored in order of descending pointers.
                loop {
                    if *free_list == EMPTY_FREE_LIST {
                        (*ptr).next = EMPTY_FREE_LIST;
                        (*ptr).size = size;
                        *free_list = ptr;
                        return;
                    }

                    if *free_list == after_new {
                        // Merge new node into node after this one.

                        let new_size = size + (**free_list).size;
                        let next = (**free_list).next;
                        if next != EMPTY_FREE_LIST && offset_bytes(next, (*next).size) == ptr {
                            // Merge into node before this one, as well as after it.
                            (*next).size += new_size;
                            // Sine we are combining 2 existing nodes (with the new one in-between)
                            // remove one from the list.
                            *free_list = next;
                            return;
                        }
                        // Edit node in free list, moving its location and updating its size.
                        *free_list = ptr;
                        (*ptr).size = new_size;
                        (*ptr).next = next;
                        return;
                    }

                    if *free_list < ptr {
                        // Merge onto end of current if adjacent
                        if offset_bytes(*free_list, (**free_list).size) == ptr {
                            // Merge into node before this one, as well as after it.
                            (**free_list).size += size;
                            // Sine we are combining the new node into the end of an existing node, no pointer updates, just a size change.
                            return;
                        }
                        // Create a new free list node
                        (*ptr).next = *free_list;
                        (*ptr).size = size;
                        *free_list = ptr;
                        return;
                    }
                    free_list = ptr::addr_of_mut!((**free_list).next);
                }
            }
        }
    }

    fn full_size(layout: Layout) -> usize {
        let grown = layout.size().max(NODE_SIZE);
        round_up(grown, NODE_SIZE)
    }

    /// Round up value to the nearest multiple of increment, which must be a
    /// power of 2. If `value` is a multiple of increment, it is returned
    /// unchanged.
    fn round_up(value: usize, increment: usize) -> usize {
        debug_assert!(increment.is_power_of_two());

        // Compute `value.div_ceil(increment) * increment`,
        // in a way that takes advantage of the fact that `increment` is
        // always a power of two to avoid using an integer divide, since that
        // wouldn't always get optimized out.
        multiple_below(value + (increment - 1), increment)
    }

    /// Round down value to the nearest multiple of increment, which must be a
    /// power of 2. If `value` is a multiple of `increment`, it is returned
    /// unchanged.
    fn multiple_below(value: usize, increment: usize) -> usize {
        debug_assert!(increment.is_power_of_two());

        // Compute `value / increment * increment` in a way
        // that takes advantage of the fact that `increment` is always a power of
        // two to avoid using an integer divide, since that wouldn't always get
        // optimized out.
        value & increment.wrapping_neg()
    }

    unsafe fn offset_bytes(ptr: *mut FreeListNode, offset: usize) -> *mut FreeListNode {
        unsafe { (ptr as *mut u8).add(offset) as *mut FreeListNode }
    }
}
