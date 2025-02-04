#![feature(c_size_t)]
#![feature(str_from_raw_parts)]
#![no_std]
#![no_main]

mod allocator;
mod collections;
mod io;

use crate::collections::{String};
use crate::io::{print, println};
use allocator::DummyAllocator;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOC: DummyAllocator = DummyAllocator;


#[no_mangle]
extern "C" fn main() {
    let mut x = String::from("Hello");
    x.push(" world!");
    println(x);

    let y: i64 = i64::MIN;

    println(y);
    
    // 255
    // 255 % 10 = 5
    
    // 255 mod 100
    
    // 7 % 5 = 2
    // 15 % 10 = 5
    // 25 % 10 = 5
    // 354 % 10 = 4
    
    // 145325238 % 10 = 8
    
    // 30 % 9 = 3
    
    // x - before % 10^n
 
    
     // 140 / 100
    
    // x = 1142
    // Iteration 1: x % 10^n+1 = 2 | x -= 2 | | x / 10^n-1 (1) = 2 | DIGIT 1: 2 | x = 1140
    // Iteration 2: x % 100 = 40 | x -= 40 | x / 10^n-1 (10) = 4 | DIGIT 2: 4 | x = 1100
    // Iteration 3: x % 1000 = 100 | x -= 100 | x / 10^n-1 (100) = 1 | DIGIT 3: 1 | x = 1000

    
}


#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        print("Panic occurred in file ");
        print(location.file());
        print(" at line ");
        println(location.line());
    }
    println("panic:");
    println(info.message().as_str().unwrap());
    loop {}
}