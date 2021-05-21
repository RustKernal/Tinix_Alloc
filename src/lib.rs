#![no_std]
#![feature(alloc_error_handler)] // at the top of the file

extern crate alloc;
use bootloader::BootInfo;
pub mod paging;
pub mod memory;
pub mod allocator;
pub mod structures;
use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;





#[alloc_error_handler]
pub fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}

pub fn construct_allocator(_boot_info : &BootInfo) -> &'static dyn GlobalAlloc {
    &DummyAllocator
}

pub struct DummyAllocator;

unsafe impl GlobalAlloc for DummyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}

