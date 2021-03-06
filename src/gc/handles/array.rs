use gc::{ArrayLocal, GcAllocator, ptr_t};
use std::ops::{Deref, DerefMut, Index, IndexMut};
use std::marker::PhantomData;
use std::ptr;
use std::mem::{size_of, transmute};
use std::slice;
use std::fmt;

pub struct Array<T> {
    ptr: ptr_t,
    _type: PhantomData<T>
}

impl<T> Array<T> {
    pub fn ptr(&self) -> ptr_t {
        self.ptr
    }
    
    pub fn from_ptr(ptr: ptr_t) -> Array<T> {
        Array {
            ptr: ptr,
            _type: PhantomData
        }
    }
    
    pub fn null() -> Array<T> {
        Self::from_ptr(ptr::null())
    }
    
    pub fn is_null(&self) -> bool {
        self.ptr.is_null()
    }
    
    pub fn len(&self) -> usize {
        unsafe { *transmute::<_, *const usize>(self.ptr) }
    }
    
    pub fn as_local<U: GcAllocator>(&self, allocator: &U) -> ArrayLocal<T> {
        allocator.alloc_array_local_from_array(*self)
    }
}

impl<T> Copy for Array<T> {}

impl<T> Clone for Array<T> {
    fn clone(&self) -> Array<T> {
        Self::from_ptr(self.ptr)
    }
}

impl<T> fmt::Debug for Array<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "Ptr {{ ptr: {:?} }}", self.ptr)
    }
}

impl<T: Copy> Array<T> {
    pub fn copy(from: Array<T>, from_offset: usize, to: Array<T>, to_offset: usize, count: usize) {
        unsafe {
            assert!(from_offset + count <= from.len() && to_offset + count <= to.len());
            
            ptr::copy(
                from.ptr.offset((size_of::<usize>() + from_offset * size_of::<T>()) as isize),
                transmute(to.ptr.offset((size_of::<usize>() + to_offset * size_of::<T>()) as isize)),
                count * size_of::<T>()
            );
        }
    }
}

impl<T> Deref for Array<T> {
    type Target = [T];
    
    fn deref(&self) -> &[T] {
        unsafe {
            let size = *transmute::<_, *const usize>(self.ptr);
            let ptr = self.ptr.offset(size_of::<usize>() as isize);
            
            slice::from_raw_parts(
                transmute(ptr),
                size
            )
        }
    }
}

impl<T> DerefMut for Array<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe {
            let size = *transmute::<_, *const usize>(self.ptr);
            let ptr = self.ptr.offset(size_of::<usize>() as isize);
            
            slice::from_raw_parts_mut(
                transmute(ptr),
                size
            )
        }
    }
}

impl<T> Index<usize> for Array<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &<Self as Index<usize>>::Output {
        assert!(index < self.len());
        
        unsafe {
            let ptr = self.ptr.offset(size_of::<usize>() as isize);
            let ptr = transmute::<_, *const T>(ptr);
            let ptr = ptr.offset(index as isize);
            
            transmute::<_, &T>(ptr)
        }
    }
}

impl<T> IndexMut<usize> for Array<T> {
    fn index_mut(&mut self, index: usize) -> &mut <Self as Index<usize>>::Output {
        assert!(index < self.len());
        
        unsafe {
            let ptr = self.ptr.offset(size_of::<usize>() as isize);
            let ptr = transmute::<_, *const T>(ptr);
            let ptr = ptr.offset(index as isize);
            
            transmute::<_, &mut T>(ptr)
        }
    }
}

pub trait AsArray<T> {
    fn as_ptr(&self) -> Array<T>;
}

impl<T> AsArray<T> for Array<T> {
    fn as_ptr(&self) -> Array<T> {
        *self
    }
}
