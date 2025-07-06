use core::{
    alloc::GlobalAlloc,
    ffi::{c_char, c_void},
    ptr,
};

unsafe extern "C" {
    pub fn heapAlloc(heap: u32, size: u32, name: *const c_char, curr: bool) -> *mut c_void;
    pub fn heapFree(ptr: *mut c_void);
}

pub struct MnlAllocator {}

#[global_allocator]
pub static ALLOCATOR: MnlAllocator = MnlAllocator {};

unsafe impl GlobalAlloc for MnlAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let padded_layout = layout.pad_to_align();
        unsafe { heapAlloc(1, padded_layout.size() as u32, ptr::null(), false).cast() }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe {
            heapFree(ptr.cast());
        }
    }
}
