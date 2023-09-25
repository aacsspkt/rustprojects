use std::{alloc, ptr::NonNull};

pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    capacity: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self {
        Self {
            capacity: 0,
            len: 0,
            ptr: NonNull::dangling(),
        }
    }

    pub fn push(&mut self, item: T) {
        assert_ne!(std::mem::size_of::<T>(), 0);

        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4).expect("Could not allocate memory");
            let ptr = unsafe { alloc::alloc(layout) } as *mut T;
            let ptr = NonNull::new(ptr).expect("Could not allocate memory");

            unsafe {
                ptr.as_ptr().write(item);
            }

            self.ptr = ptr;
            self.capacity = 4;
            self.len = 1;
        } else if self.len < self.capacity {
            let offset = self
                .len
                .checked_mul(std::mem::size_of::<T>())
                .expect("Cannot reach memory location");
            assert!(offset < isize::MAX as usize, "Wrapped isize");
            unsafe { self.ptr.as_ptr().add(self.len).write(item) }
            self.len += 1;
        } else {
            debug_assert!(self.len == self.capacity);
            let new_capacity = self.capacity.checked_mul(2).expect("Capacity wrapped");
            let align = std::mem::align_of::<T>();
            let size = std::mem::size_of::<T>() * self.capacity;
            size.checked_add(size % align).expect("Can't allocate");

            let ptr = unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = std::mem::size_of::<T>() * new_capacity;
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size);
                let ptr = NonNull::new(ptr as *mut T).expect("Could not reallocate");

                ptr.as_ptr().add(self.len).write(item);
                ptr
            };
            self.ptr = ptr;
            self.len += 1;
            self.capacity = new_capacity;
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn capacity(&self) -> usize {
        self.capacity
    }

    pub fn get(&self, index: usize)-> Option<&T> {
        if index >= self.len {
            return None;
        }

        Some(unsafe {
            &*self.ptr.as_ptr().add(index)
        })
    }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::drop_in_place(std::slice::from_raw_parts_mut(self.ptr.as_ptr(), self.len));
            let layout = alloc::Layout::from_size_align_unchecked(
                std::mem::size_of::<T>() * self.capacity,
                std::mem::align_of::<T>(),
            );
            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut vec = MyVec::<usize>::new();
        vec.push(1usize);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);

        for n in 0..vec.len() {
            assert_eq!(vec.get(n), Some(&(n + 1)))
        }

        assert_eq!(vec.len(), 5);
        assert_eq!(vec.capacity(), 8);
    }
}
