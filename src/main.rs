#![feature(c_size_t)]
#![feature(str_from_raw_parts)]
#![no_std]
#![no_main]

mod allocator;
mod collections;
mod io;

use crate::collections::{String, Vec};
use crate::io::println;
use allocator::DummyAllocator;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOC: DummyAllocator = DummyAllocator;


#[no_mangle]
extern "C" fn main() {
    let mut x = String::from("Hello");
    x.push(" world!");
    println(x);
    
    let mut vec: Vec<&str> = Vec::with_capacity(1);
    vec.push("a");
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println("panic:");
    println(info.message().as_str().unwrap());
    loop {}
}