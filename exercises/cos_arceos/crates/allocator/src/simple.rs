//! Simple memory allocation.
//!
//! TODO: more efficient

use core::alloc::Layout;
use core::num::NonZeroUsize;

use crate::{AllocResult, BaseAllocator, ByteAllocator};

pub struct SimpleByteAllocator {
    start: usize,
    size: usize,
    used: usize,
}

impl SimpleByteAllocator {
    pub const fn new() -> Self {
        Self {
            start: 0,
            size: 0,
            used: 0,
        }
    }
}

impl BaseAllocator for SimpleByteAllocator {
    fn init(&mut self, start: usize, size: usize) {
        self.start = start;
        self.size = size;
        self.used = 0;
    }

    fn add_memory(&mut self, start: usize, size: usize) -> AllocResult {
        if start + size > self.start + self.size {
            return Err(crate::AllocError::NoMemory);
        }
        self.size += size;
        Ok(())
    }
}

impl ByteAllocator for SimpleByteAllocator {
    fn alloc(&mut self, layout: Layout) -> AllocResult<NonZeroUsize> {
        let size = layout.size();
        let align = layout.align();

        let padding = align - (self.start + self.used) % align;
        let pos = self.start + self.used + padding;

        if pos + size > self.start + self.size {
            return Err(crate::AllocError::NoMemory);
        }

        self.used += padding + size;

        Ok(unsafe { NonZeroUsize::new_unchecked(pos) })
    }

    fn dealloc(&mut self, pos: NonZeroUsize, layout: Layout) {
        let size = layout.size();
        let align = layout.align();

        let padding = align - (self.start + self.used) % align;
        let expected_pos = self.start + self.used + padding - size;

        debug_assert_eq!(pos.get(), expected_pos);

        self.used -= padding + size;
    }

    fn total_bytes(&self) -> usize {
        self.size
    }

    fn used_bytes(&self) -> usize {
        self.used
    }

    fn available_bytes(&self) -> usize {
        self.size - self.used
    }
}
