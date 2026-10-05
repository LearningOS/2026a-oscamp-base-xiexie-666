#![cfg_attr(not(test), no_std)]
use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

struct FreeBlock { size: usize, next: *mut FreeBlock }

pub struct FreeListAllocator {
    heap_start: usize,
    heap_end: usize,
    bump_next: core::sync::atomic::AtomicUsize,
    #[cfg(test)] free_list: std::sync::Mutex<*mut FreeBlock>,
    #[cfg(not(test))] free_list: core::cell::UnsafeCell<*mut FreeBlock>,
}
#[cfg(test)] unsafe impl Send for FreeListAllocator {}
#[cfg(test)] unsafe impl Sync for FreeListAllocator {}
#[cfg(not(test))] unsafe impl Send for FreeListAllocator {}
#[cfg(not(test))] unsafe impl Sync for FreeListAllocator {}

impl FreeListAllocator {
    pub unsafe fn new(heap_start: usize, heap_end: usize) -> Self {
        Self { heap_start, heap_end, bump_next: core::sync::atomic::AtomicUsize::new(heap_start),
            #[cfg(test)] free_list: std::sync::Mutex::new(null_mut()),
            #[cfg(not(test))] free_list: core::cell::UnsafeCell::new(null_mut()) }
    }
    #[cfg(test)] fn free_list_head(&self) -> *mut FreeBlock { *self.free_list.lock().unwrap() }
    #[cfg(test)] fn set_free_list_head(&self, head: *mut FreeBlock) { *self.free_list.lock().unwrap() = head; }
    #[cfg(not(test))] fn free_list_head(&self) -> *mut FreeBlock { unsafe { *self.free_list.get() } }
    #[cfg(not(test))] fn set_free_list_head(&self, head: *mut FreeBlock) { unsafe { *self.free_list.get() = head; } }
}

unsafe impl GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size().max(core::mem::size_of::<FreeBlock>());
        let align = layout.align().max(core::mem::align_of::<FreeBlock>());
        let mut prev: *mut FreeBlock = null_mut();
        let mut curr = self.free_list_head();
        while !curr.is_null() {
            let next = (*curr).next;
            if (curr as usize) % align == 0 && (*curr).size >= size {
                if prev.is_null() { self.set_free_list_head(next); } else { (*prev).next = next; }
                return curr as *mut u8;
            }
            prev = curr; curr = next;
        }
        let mut current = self.bump_next.load(core::sync::atomic::Ordering::SeqCst);
        loop {
            let aligned = match current.checked_add(align - 1) { Some(v) => v & !(align - 1), None => return null_mut() };
            let end = match aligned.checked_add(size) { Some(v) if v <= self.heap_end => v, _ => return null_mut() };
            match self.bump_next.compare_exchange(current, end, core::sync::atomic::Ordering::SeqCst, core::sync::atomic::Ordering::SeqCst) { Ok(_) => return aligned as *mut u8, Err(v) => current = v }
        }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let block = ptr as *mut FreeBlock;
        block.write(FreeBlock { size: layout.size().max(core::mem::size_of::<FreeBlock>()), next: self.free_list_head() });
        self.set_free_list_head(block);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const HEAP_SIZE: usize = 4096;

    fn make_allocator() -> (FreeListAllocator, Vec<u8>) {
        let mut heap = vec![0u8; HEAP_SIZE];
        let start = heap.as_mut_ptr() as usize;
        let alloc = unsafe { FreeListAllocator::new(start, start + HEAP_SIZE) };
        (alloc, heap)
    }

    #[test]
    fn test_alloc_basic() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(32, 8).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(!ptr.is_null());
    }

    #[test]
    fn test_alloc_alignment() {
        let (alloc, _heap) = make_allocator();
        for align in [1, 2, 4, 8, 16] {
            let layout = Layout::from_size_align(8, align).unwrap();
            let ptr = unsafe { alloc.alloc(layout) };
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % align, 0, "align={align}");
        }
    }

    #[test]
    fn test_dealloc_and_reuse() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(64, 8).unwrap();

        let p1 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null());

        // After freeing, the next allocation should reuse the same block
        unsafe { alloc.dealloc(p1, layout) };
        let p2 = unsafe { alloc.alloc(layout) };
        assert!(!p2.is_null());
        assert_eq!(p1, p2, "should reuse the freed block");
    }

    #[test]
    fn test_multiple_alloc_dealloc() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(128, 8).unwrap();

        let p1 = unsafe { alloc.alloc(layout) };
        let p2 = unsafe { alloc.alloc(layout) };
        let p3 = unsafe { alloc.alloc(layout) };
        assert!(!p1.is_null() && !p2.is_null() && !p3.is_null());

        unsafe { alloc.dealloc(p2, layout) };
        unsafe { alloc.dealloc(p1, layout) };

        let q1 = unsafe { alloc.alloc(layout) };
        let q2 = unsafe { alloc.alloc(layout) };
        assert!(!q1.is_null() && !q2.is_null());
    }

    #[test]
    fn test_oom() {
        let (alloc, _heap) = make_allocator();
        let layout = Layout::from_size_align(HEAP_SIZE + 1, 1).unwrap();
        let ptr = unsafe { alloc.alloc(layout) };
        assert!(ptr.is_null(), "should return null when exceeding heap");
    }
}
