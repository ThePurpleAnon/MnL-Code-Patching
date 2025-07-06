use core::{alloc::GlobalAlloc, ffi::c_void};

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum OSArenaId {
    OS_ARENA_MAIN = 0,
    OS_ARENA_MAIN_SUBPRIV = 1,
    OS_ARENA_MAINEX = 2,
    OS_ARENA_ITCM = 3,
    OS_ARENA_DTCM = 4,
    OS_ARENA_SHARED = 5,
    OS_ARENA_WRAM_MAIN = 6,
    OS_ARENA_WRAM_SUB = 7,
    OS_ARENA_WRAM_SUBPRIV = 8,

    OS_ARENA_MAX = 9,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OSHeapHandle(pub i32);
pub const OS_CURRENT_HEAP_HANDLE: OSHeapHandle = OSHeapHandle(-1);

unsafe extern "C" {
    pub fn OS_AllocFromHeap(id: OSArenaId, heap: OSHeapHandle, size: u32) -> *mut c_void;
    pub fn OS_FreeToHeap(id: OSArenaId, heap: OSHeapHandle, ptr: *mut c_void);
}

pub struct NitroSdkAllocator {}

// #[global_allocator]
pub static ALLOCATOR: NitroSdkAllocator = NitroSdkAllocator {};

unsafe impl GlobalAlloc for NitroSdkAllocator {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        let padded_layout = layout.pad_to_align();
        unsafe {
            OS_AllocFromHeap(
                OSArenaId::OS_ARENA_MAIN,
                OS_CURRENT_HEAP_HANDLE,
                padded_layout.size() as u32,
            )
            .cast()
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: core::alloc::Layout) {
        unsafe {
            OS_FreeToHeap(OSArenaId::OS_ARENA_MAIN, OS_CURRENT_HEAP_HANDLE, ptr.cast());
        }
    }
}
