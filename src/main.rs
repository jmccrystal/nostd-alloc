#![feature(c_size_t)]
#![feature(str_from_raw_parts)]
#![no_std]
#![no_main]

mod allocator;

use core::alloc::{GlobalAlloc, Layout};
use core::cmp::max;
use core::ffi::c_void;
use core::ops::{Deref, DerefMut};
use core::panic::PanicInfo;
use core::ptr::{null, null_mut, slice_from_raw_parts, slice_from_raw_parts_mut};
use allocator::DummyAllocator;
use crate::allocator::PAGE_SIZE;

#[global_allocator]
static ALLOC: DummyAllocator = DummyAllocator;

extern "C" {
    fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
    fn read(fd: i32, buf: *mut c_void, count: usize) -> isize;
}

/// # Safety
/// Caller must ensure:
/// - Size does not exceed allocated memory
/// - Layout alignment is respected
/// - Returned pointer is not used after free()
/// - No other references to this memory exist
///
/// # Panics
/// Will panic if size > isize::MAX
#[must_use] pub unsafe fn malloc(size: usize) -> *mut u8 {
    assert!(isize::try_from(size).is_ok());
    
    // Unwrap to panic in the case of memory issue, rather than returning null ptr
    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe { ALLOC.alloc(layout) }
}

// /// # Safety
// /// Caller must ensure:
// /// - Pointer must be from malloc()
// /// - Size matches original allocation
// /// - No other references exist
// /// - Memory not already freed
// /// - Memory not used after free
// // TODO: uncomment once dealloc() is implemented
// pub unsafe fn free<T>(addr: *mut T, size: usize) {
//     assert!(isize::try_from(size).is_ok());
//     let layout = Layout::from_size_align(size, 1).unwrap();
//     unsafe { ALLOC.dealloc(addr.cast::<u8>(), layout) }
// }

pub struct Vec<T> {
    offset: usize,
    capacity: usize,
    addr: *mut T,
}

impl<T> Vec<T> {
    fn with_capacity(capacity: usize) -> Self {
        let addr = unsafe { malloc(capacity).cast::<T>() };
        Self { offset: 0, capacity, addr }
    }
    fn push(&mut self, data: T) {
        // Resize if push will exceed capacity
        if self.offset + size_of::<T>() > self.capacity {
            self.resize(size_of::<T>());
        }
        let addr = ((self.addr as usize) + self.offset) as *mut T;
        unsafe { *addr = data };
        self.offset += size_of::<T>();
    }
    fn push_slice(&mut self, data: &[T]) where T: Clone {
        // push each value individually
        for val in data {
            self.push(val.clone());
        }
    }
    fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.addr, self.offset) }
    }
    fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self.addr, self.offset) }
    }
    fn resize(&mut self, needed_size: usize) {
        // in case resize over doubles the size
        let new_size = max(self.capacity * 2, needed_size);
        // If resize is necessary, reallocate and increase size
        let new_vec = Self::with_capacity(new_size);
        unsafe {
            // Copy data from old vec to new vec
            core::ptr::copy_nonoverlapping(self.addr, new_vec.addr, self.offset);
            // free(self.addr, self.offset); TODO: free memory
        }
        self.addr = new_vec.addr;
        self.capacity = new_vec.capacity;
    }
    #[must_use] pub fn as_ptr(&self) -> *const T {
        self.addr
    }
    #[must_use] pub fn len(&self) -> usize {
        self.offset
    }
}

impl<T> Clone for Vec<T> {
    fn clone(&self) -> Self {
        // Allocate new vec
        let new_vec = Self::with_capacity(self.capacity);
        // Copy data into new vec
        unsafe { core::ptr::copy_nonoverlapping(self.addr, new_vec.addr, self.offset) };
        new_vec
    }
}

// Inner items must be printable and cloneable
impl<T: Clone + Printable> Printable for Vec<T> {
    fn print(&self) {
        for val in self.as_slice() {
            print(val.clone());
        }
    }
}

pub struct String {
    vec: Vec<u8>
}

impl String {
    // Pushes utf 8 single bytes
    fn push(&mut self, str: &str) {
        self.vec.push_slice(str.as_bytes());
    }
}

impl Printable for String {
    fn print(&self) {
        let str = unsafe { core::str::from_raw_parts(self.vec.as_ptr(), self.vec.len()) };
        print(str);
    }
}

impl From<&str> for String {
    fn from(value: &str) -> Self {
        let mut vec = Vec::with_capacity(value.len());
        vec.push_slice(value.as_bytes());
        String { vec }
    }
}

impl Deref for String {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl DerefMut for String {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

trait Printable {
    fn print(&self);
    fn println(&self) {
        self.print();
        "\n".print();
    } 
}

impl Printable for &str {
    /// Writes a string to stdout. Returns number of bytes written, or negative value on error.
    ///
    /// # Safety
    /// The function passes fd: 1 to write which will always exist
    /// msg will always be a valid reference which prevents dangling pointers
    /// Will always pass the correct length
    fn print(&self) {
        unsafe { write(1, self.as_ptr().cast::<c_void>(), self.len()) };
    }
}

fn print(msg: impl Printable) {
    msg.print();
}

fn println(msg: impl Printable) {
    print(msg);
    print("\n");
}

#[no_mangle]
extern "C" fn main() {
    let mut x = String::from("Hello");
    let mut vec: Vec<u8> = Vec::with_capacity(1);
    vec.push(2);
    vec.push(3);
    x.push(" world!");
    println(x);
}

#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    println("panic:");
    println(info.message().as_str().unwrap());
    loop {}
}