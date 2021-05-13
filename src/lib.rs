#![no_std]
#![feature(decl_macro)]
#![feature(abi_x86_interrupt)]


extern crate alloc;
pub mod paging;
pub mod memory;
pub mod heap;