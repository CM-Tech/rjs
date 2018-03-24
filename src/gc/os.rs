// See https://github.com/jemalloc/jemalloc/blob/dev/src/chunk_mmap.c for how jemalloc allocates memory.

extern crate libc;
extern crate memalloc;
use self::libc::*;
use std::ptr;
use std::mem;
use gc::ptr_t;


pub struct Memory {
    ptr: *mut u8,
    size: usize
}

impl Memory {
    pub fn empty() -> Memory {
        Memory {
            ptr: ptr::null_mut(),
            size: 0
        }
    }

    pub fn alloc(size: usize) -> Option<Memory> {
        let ptr = unsafe {memalloc::allocate(size)};
        if ptr.is_null() {
            None
        } else {
            Some(Memory {
                ptr: ptr,
                size: size
            })
        }
    }

    pub unsafe fn ptr(&self) -> ptr_t {
        self.ptr
    }

    pub fn size(&self) -> usize {
        self.size
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        if self.size > 0 {
            unsafe{memalloc::deallocate(self.ptr, self.size)} ;
        }
    }
}
