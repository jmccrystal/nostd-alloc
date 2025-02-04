use core::array::IntoIter;
use core::cmp::max;
use crate::allocator::malloc;
use crate::io::{Printable, print};

pub struct Vec<T> {
    // current location of vec pointer
    offset: usize,
    // size of vec in bytes
    capacity: usize,
    // base address
    addr: *mut T,
}

// TODO: implement Iterator
// TODO: add access by index
impl<T> Vec<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        let addr = unsafe { malloc(capacity).cast::<T>() };
        Self { offset: 0, capacity, addr }
    }
    // TODO: `size_of()` func is being used as byte count and length, making every vec with
    // element size of anything other 1 not work.
    pub fn push(&mut self, data: T) {
        // Resize if push will exceed capacity
        if self.offset + size_of::<T>() > self.capacity {
            self.resize(size_of::<T>());
        }
        let addr = ((self.addr as usize) + self.offset) as *mut T;
        unsafe { *addr = data };
        self.offset += size_of::<T>();
    }
    pub fn push_slice(&mut self, data: &[T]) where T: Clone {
        // push each value individually
        for val in data {
            self.push(val.clone());
        }
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.addr, self.offset) }
    }
    pub fn as_mut_slice(&mut self) -> &mut [T] {
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

// impl<T> Iterator for Vec<T> {
//     type Item = T;
// 
//     // TODO: this is wrong, need to know where it is?
//     fn next(&mut self) -> Option<Self::Item> {
//         let new_addr = self.offset += size_of::<T>();
//         
//     }
// }