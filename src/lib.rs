use std::alloc::{GlobalAlloc, Layout};

mod alloc;
use alloc::*;

pub struct Shalloc;

unsafe impl GlobalAlloc for Shalloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        alloc_alloc_e(
            size.max(align),
            0,
        ) as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let size = layout.size();
        let align = layout.align();
        alloc_free_e(
            ptr as _,
            size.max(align),
        )
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();
        alloc_alloc_e(
            size.max(align),
            1,
        ) as *mut u8
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old_size = layout.size();
        let align = layout.align();
        alloc_realloc_e(
            ptr as _,
            old_size.max(align),
            new_size.max(align),
            0,
        ) as *mut u8
    }
}
