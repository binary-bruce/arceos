use super::{AllocResult, BaseAllocator, ByteAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;

pub struct BumpAllocator;

impl BumpAllocator {
    pub const fn new() -> Self {
        Self
    }
}

impl BaseAllocator for BumpAllocator {
    fn init(&mut self, start: usize, size: usize) {
        unimplemented!()
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        unimplemented!()
    }
}

impl ByteAllocator for BumpAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        unimplemented!()
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unimplemented!()
    }

    fn total_bytes(&self) -> usize {
        unimplemented!()
    }

    fn used_bytes(&self) -> usize {
        unimplemented!()
    }

    fn available_bytes(&self) -> usize {
        unimplemented!()
    }
}
