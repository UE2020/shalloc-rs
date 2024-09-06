use std::alloc::{GlobalAlloc, Layout};

mod alloc;
use alloc::*;

pub struct Shalloc;

unsafe impl GlobalAlloc for Shalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        AllocAllocH(
            AllocGetHandleS(
                AllocGetGlobalState(),
                layout.size().max(layout.align()) as _,
            ),
            layout.size().max(layout.align()) as _,
            0,
        ) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        AllocFreeH(
            AllocGetHandleS(
                AllocGetGlobalState(),
                layout.size().max(layout.align()) as _,
            ),
            ptr as _,
            layout.size().max(layout.align()) as _,
        );
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        AllocAllocH(
            AllocGetHandleS(
                AllocGetGlobalState(),
                layout.size().max(layout.align()) as _,
            ),
            layout.size().max(layout.align()) as _,
            1,
        ) as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        AllocReallocH(
            AllocGetHandleS(
                AllocGetGlobalState(),
                layout.size().max(layout.align()) as _,
            ),
            ptr as _,
            layout.size().max(layout.align()) as _,
            AllocGetHandleS(AllocGetGlobalState(), new_size.max(layout.align()) as _),
            new_size.max(layout.align()) as _,
            0,
        ) as *mut u8
    }
}
