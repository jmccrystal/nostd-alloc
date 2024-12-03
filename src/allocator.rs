use core::alloc::{GlobalAlloc, Layout};
use core::ffi::c_void;
use core::ptr::null_mut;
use core::sync::atomic::Ordering::SeqCst;
use core::sync::atomic::{AtomicPtr, AtomicUsize};

static BASE_ADDR: AtomicPtr<u8> = AtomicPtr::new(null_mut());
static OFFSET: AtomicUsize = AtomicUsize::new(0);

// 4096 bytes
pub(crate) const PAGE_SIZE: usize = 0x1000;
const PROT_READ: i32 = 0x1;
const PROT_WRITE: i32 = 0x2;
const MAP_PRIVATE: i32 = 0x0002;
const MAP_ANON: i32 = 0x1000;


pub struct DummyAllocator;

extern "C" {
    fn mmap(
        addr: *mut c_void,
        length: usize, 
        prot: i32, 
        flags: i32, 
        fd: i32, 
        offset: i64
    ) -> *mut c_void;
}
unsafe impl GlobalAlloc for DummyAllocator {
    /// Allocate a chunk of memory if current memory chunk is full or has not yet been initialized.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // TODO: This returns a memory address. Set the mem addr to the returned address + length of mem requested
        
        let mut addr = BASE_ADDR.load(SeqCst);
        let offset = OFFSET.load(SeqCst);
        let size = layout.size();
        
        if size > PAGE_SIZE {
            // TODO: handle heap allocations larger than 4096 bytes
            panic!();
        }
        
        // if memory chunk has not yet been allocated
        // or if the current offset points to a not yet allocated section of memory
        if addr.is_null() || offset > PAGE_SIZE - size {
            addr = mmap(
                null_mut(),
                PAGE_SIZE, 
                PROT_READ | PROT_WRITE,
                MAP_PRIVATE | MAP_ANON, 
                -1, 
                0
            ).cast::<u8>();
            BASE_ADDR.store(addr, SeqCst);
            OFFSET.store(0, SeqCst);
        }
        // Store new offset as current offset + size of chunk allocated
        OFFSET.store(offset + size, SeqCst);
        
        // returns base address + current offset
        (addr as usize + offset) as *mut u8
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        
    }
}


