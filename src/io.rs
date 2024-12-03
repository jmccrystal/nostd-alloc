use core::ffi::c_void;

extern "C" {
    fn write(fd: i32, buf: *const c_void, count: usize) -> isize;
}

pub trait Printable {
    fn print(&self);
    fn println(&self) {
        self.print();
        "\n".print();
    }
}

impl<T: AsRef<[u8]>> Printable for T {
    fn print(&self) {
        let bytes = self.as_ref();
        unsafe { write(1, bytes.as_ptr().cast::<c_void>(), bytes.len()) };
    }
}

pub fn print(msg: impl Printable) {
    msg.print();
}

pub fn println(msg: impl Printable) {
    print(msg);
    print("\n");
}

// TODO: implement `Printable` for primitives like i32