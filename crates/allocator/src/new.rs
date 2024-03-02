use super::{AllocError, AllocResult, BaseAllocator, ByteAllocator};
use core::alloc::Layout;
use core::ptr::NonNull;
use talc::*;

pub struct YourNewByteAllocator {
    inner: Talc<ErrOnOom>,
    total_bytes: usize,
    used_bytes: usize,
}

impl YourNewByteAllocator {
    pub const fn new() -> Self {
        Self {
            inner: Talc::new(ErrOnOom),
            total_bytes: 0,
            used_bytes: 0,
        }
    }
}

impl BaseAllocator for YourNewByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        let memory = Span::new(start as *mut u8, (start + size) as *mut u8);
        unsafe {
            let _ = self.inner.claim(memory); // assume it woun't fail
        };
        self.total_bytes += size;
    }

    fn add_memory(&mut self, _start: usize, _size: usize) -> AllocResult {
        let memory = Span::new(_start as *mut u8, (_start + _size) as *mut u8);
        unsafe {
            let _ = self.inner.claim(memory); // assume it woun't fail
        };
        self.total_bytes += _size;

        Ok(())
    }
}

impl ByteAllocator for YourNewByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        unsafe {
            self.inner
                .malloc(layout)
                .map(|addr| {
                    self.used_bytes -= layout.size();
                    addr
                })
                .map_err(|_| AllocError::NoMemory)
        }
    }

    fn dealloc(&mut self, pos: NonNull<u8>, layout: Layout) {
        unsafe { self.inner.free(pos, layout) }

        self.used_bytes += layout.size()
    }

    fn total_bytes(&self) -> usize {
        self.total_bytes
    }

    fn used_bytes(&self) -> usize {
        self.used_bytes
    }

    fn available_bytes(&self) -> usize {
        self.total_bytes - self.used_bytes
    }
}
