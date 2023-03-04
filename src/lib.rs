#![cfg_attr(not(test), no_std)]
#![feature(alloc_error_handler)]
#![feature(vec_into_raw_parts)]

mod allocator;
mod compiler;
mod panic_handler;
mod parser;
