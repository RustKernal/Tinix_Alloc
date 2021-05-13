#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]
#![feature(alloc_error_handler)] // at the top of the file

extern crate alloc;

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub mod paging;
pub mod memory;
pub mod heap;