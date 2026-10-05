#![cfg_attr(not(test), no_std)]

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use core::sync::atomic::{AtomicUsize, Ordering};

pub struct BumpAllocator { heap_start: usize, heap_end: usize, next: AtomicUsize }

impl BumpAllocator {
    pub const unsafe fn new(heap_start: usize, heap_end: usize) -> Self {
        Self { heap_start, heap_end, next: AtomicUsize::new(heap_start) }
    }
    pub fn reset(&self) { self.next.store(self.heap_start, Ordering::SeqCst); }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let align = layout.align();
        let size = layout.size();
        let mut current = self.next.load(Ordering::SeqCst);
        loop {
            let aligned = match current.checked_add(align - 1) { Some(v) => v & !(align - 1), None => return null_mut() };
            let end = match aligned.checked_add(size) { Some(v) if v <= self.heap_end => v, _ => return null_mut() };
            match self.next.compare_exchange(current, end, Ordering::SeqCst, Ordering::SeqCst) {
                Ok(_) => return aligned as *mut u8,
                Err(actual) => current = actual,
            }
        }
    }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
