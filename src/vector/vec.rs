use std::mem::{size_of, align_of};
use std::ptr::NonNull;
use std::alloc;


#[derive(Debug)]
pub struct Vector<T> {
    ptr: NonNull<T>,
    capacity: usize,
    length: usize,
}


impl<T> Vector<T> {
    pub fn new() -> Self {
        return Self {
            ptr: NonNull::dangling(),
            capacity: 0,
            length: 0,
        }
    }

    pub fn capacity(&self) -> usize {
        return self.capacity;
    }

    pub fn len(&self) -> usize {
        return self.length;
    }

    pub fn push(&mut self, value: T) {
        assert_ne!(size_of::<T>(), 0, "Zero-sized Types are not allowed.");
        
        if self.capacity == 0 {
            let layout = alloc::Layout::array::<T>(4)
                .expect("Could not allocate memory.");
            
            let pointer = NonNull::new(unsafe { alloc::alloc(layout) } as *mut T)
                .expect("Could not allocate memory.");

            unsafe { pointer.as_ptr().write(value); }

            self.ptr = pointer;
            self.capacity = 4;
            self.length = 1;
        } else if self.length < self.capacity {
            /* Note: Check may not be necessary. */
            let offset = self.length
                .checked_mul(size_of::<T>())
                .expect("Cannot reach memory location.");
            assert!(offset < isize::MAX as usize, "Wrapped isize.");
            
            unsafe { self.ptr.as_ptr().add(self.length).write(value); }
            self.length += 1;
        } else {
            debug_assert!(self.length == self.capacity);
            let new_capacity = self.capacity.checked_mul(2)
                .expect("Capacity wrapped.");
            
            let size = size_of::<T>() * self.capacity;
            let align = align_of::<T>();

            size.checked_add(size % align)
                .expect("Cannot allocate memory.");

            unsafe {
                let layout = alloc::Layout::from_size_align_unchecked(size, align);
                let new_size = size_of::<T>() * new_capacity;

                let pointer = NonNull::new(alloc::realloc(self.ptr.as_ptr() as *mut u8, layout, new_size) as *mut T)
                    .expect("Could not re-allocate memory.");
                pointer.as_ptr().add(self.length).write(value);

                self.ptr = pointer;
                self.capacity = new_capacity;
                self.length += 1;
            }
        }
    }
}


impl<T> Default for Vector<T> {
    fn default() -> Self {
        return Self::new();
    }
}


#[cfg(test)]
mod tests {
    use super::Vector;

    #[test]
    fn create_integer_vector() {
        let mut vec: Vector<i32> = Vector::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        vec.push(4);
        vec.push(5);
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn create_float_vector() {
        let mut vec: Vector<f32> = Vector::new();
        vec.push(1.0);
        vec.push(2.0);
        vec.push(3.0);
        vec.push(4.0);
        vec.push(5.0);
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }

    #[test]
    fn create_str_vector() {
        let mut vec: Vector<&str> = Vector::new();
        vec.push("One");
        vec.push("Two");
        vec.push("Three");
        vec.push("Four");
        vec.push("Five");
        assert_eq!(vec.capacity(), 8);
        assert_eq!(vec.len(), 5);
    }
}