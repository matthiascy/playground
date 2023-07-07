use std::alloc::{self, Layout};
use std::marker::PhantomData;
use std::mem;
use std::ops::{Deref, DerefMut};
use std::ptr::{self, NonNull};

// *mut T but non-zero and covariant
//
// struct NonNull<T: ?Sized> {
//     pointer: *const T,  // Covariant over T
// }
//
// struct Unique<T: ?Sized> {
//     ptr: NonNull<T>,
//     _marker: PhantomData<T>,  // with drop check (owns T)
// }
//
// Unique is a wrapper around a raw pointer that declares that:
//
// * we are covariant over T
// * we may own a value of type T (for drop check)
// * we are Send/Sync if T is Send/Sync
// * our pointer is never null (so Option<Vec<T>>) is null-pointer-optimized)
pub struct Vec<T> {
    ptr: NonNull<T>, // *mut T but non-zero and covariant
    cap: usize,
    len: usize,
    _marker: PhantomData<T>,
}

unsafe impl<T: Send> Send for Vec<T> {}
unsafe impl<T: Sync> Sync for Vec<T> {}

impl<T> Vec<T> {
    pub fn new() -> Self {
        assert!(mem::size_of::<T>() != 0, "We're not ready to handle ZSTs");
        Vec {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
            _marker: PhantomData,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap {
            self.grow();
        }

        // We can't just index to the memory and dereference it, because that will
        // evaluate the memory as a valid instance of T.
        // Worse, foo[idx] = x will try to call `drop` on the old value of foo[idx].
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), elem);
        }
        // unsafe { *self.ptr.as_mut() = elem; } // wrong, cause drop

        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe { Some(ptr::read(self.ptr.as_ptr().add(self.len))) }
        }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap == self.len {
            self.grow();
        }

        unsafe {
            ptr::copy(
                self.ptr.as_ptr().add(index),
                self.ptr.as_ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr.as_ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index <= self.len, "index out of bounds");
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr.as_ptr().add(index));
            ptr::copy(
                self.as_ptr().add(index + 1),
                self.ptr.as_ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}

impl<T> Vec<T> {
    fn grow(&mut self) {
        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            // This can't overflow since self.cap <= isize::MAX.
            let new_cap = 2 * self.cap;

            // `Layout::array` checks that the number of bytes is <= usize::MAX,
            // but this is redundant since old_layout.size() <= isize::MAX,
            // so the `unwrap` should never fail.
            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        // Ensure that the new allocation doesn't exceed `isize::MAX` bytes.
        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        // If allocation fails, `new_ptr` will be null, in which case we abort
        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };

        self.cap = new_cap
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            while let Some(_) = self.pop() {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len) }
    }
}

// Takes ownership from a Vec
pub struct VecIntoIter<T> {
    buf: NonNull<T>,
    cap: usize,
    start: *const T,
    end: *const T,
    _marker: PhantomData<T>,
}

impl<T> Drop for VecIntoIter<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            for _ in &mut *self {}
            let layout = Layout::array::<T>(self.cap).unwrap();
            unsafe { alloc::dealloc(self.buf.as_ptr() as *mut u8, layout) }
        }
    }
}

impl<T> Iterator for VecIntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                let result = ptr::read(self.start);
                self.start = self.start.offset(1);
                Some(result)
            }
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = (self.end as usize - self.start as usize) / mem::size_of::<T>();
        (len, Some(len))
    }
}

impl<T> DoubleEndedIterator for VecIntoIter<T> {
    fn next_back(&mut self) -> Option<T> {
        if self.start == self.end {
            None
        } else {
            unsafe {
                self.end = self.end.offset(-1);
                Some(ptr::read(self.end))
            }
        }
    }
}

impl<T> IntoIterator for Vec<T> {
    type Item = T;
    type IntoIter = VecIntoIter<T>;

    fn into_iter(self) -> VecIntoIter<T> {
        // Can't destructure Vec since it's Drop
        let ptr = self.ptr;
        let cap = self.cap;
        let len = self.len;

        // Make sure not to drop Vec since that would free the buffer
        mem::forget(self);

        unsafe {
            VecIntoIter {
                buf: ptr,
                cap,
                start: ptr.as_ptr(),
                end: if cap == 0 {
                    ptr.as_ptr()
                } else {
                    ptr.as_ptr().add(len)
                },
                _marker: PhantomData,
            }
        }
    }
}

pub fn run_vec() {
    println!("run_vec");
    struct A {
        a: u32,
        b: u32,
    }

    impl A {
        pub fn new(a: u32, b: u32) -> A {
            A { a, b }
        }
    }

    impl Drop for A {
        fn drop(&mut self) {
            println!("drop A {} {}", self.a, self.b)
        }
    }

    let mut v: Vec<A> = Vec::new();
    v.push(A::new(10, 20));
    v.push(A::new(20, 30));
}
