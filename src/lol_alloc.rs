// modified version of https://github.com/Craig-Macomber/lol_alloc, MIT License

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

#[cfg(target_arch = "wasm32")]
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

// Safety: No one besides us has the raw pointer, so we can safely transfer the
// FreeListAllocator to another thread.
unsafe impl Send for LolAlloc {}

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
