use core::ops::{Deref, DerefMut};
use crate::collections::Vec;
use crate::io::{print, Printable};

pub struct String {
    vec: Vec<u8>
}

impl String {
    // Pushes utf-8 single bytes
    pub fn push(&mut self, str: &str) {
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