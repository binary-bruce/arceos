use super::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    pub const fn new() -> Self {
        Self {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
}

impl BaseAllocator for BumpAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.heap_start = start;
        self.heap_end = start + size;
        self.next = start;
    }

    /// Seems like bump allocator does not support add memory that is not adjacent to the existing heap
    /// Just abandon the existing heap(heap_start..heap_end) if add_memory is called
    /// BUT THIS IS WRONG!
    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        self.init(_start, _size);
        Ok(())
    }
}

impl ByteAllocator for BumpAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        if self.available_bytes() < layout.size() {
            Err(AllocError::NoMemory)
        } else {
            let alloc_start = self.next;
            self.next = alloc_start + layout.size();
            self.allocations += 1;

            let addr = unsafe { NonNull::new_unchecked(alloc_start as *mut u8) };
            Ok(addr)
        }
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        self.allocations -= 1;
        if self.allocations == 0 {
            self.next = self.heap_start
        }
    }

    fn total_bytes(&self) -> usize {
        self.heap_end - self.heap_start
    }

    fn used_bytes(&self) -> usize {
        self.next - self.heap_start
    }

    fn available_bytes(&self) -> usize {
        self.heap_end - self.next
    }
}
