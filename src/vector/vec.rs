use std::slice::from_raw_parts_mut as slice_from_raw_parts_mut;
use std::ptr::{NonNull, drop_in_place};
use std::mem::{size_of, align_of};
use std::option::Option;
use std::alloc;


/// Shorthand Syntax for creating a new `Vector`.
/// ## Example:
/// ```rust
/// let mut vector: Vector<u8> = Vector::new();
/// vector.push(1);
/// vector.push(2);
/// vector.push(3);
/// assert_eq!(vector, vector![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! vector {
    ($($e:expr), *) => {
        {
            #[allow(unused_mut)]
            let mut vec = $crate::vector::vec::Vector::new();
            $(
                vec.push($e);
            )*
            vec
        }
    };
}


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

    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.length {
            unsafe { return Some(&*self.ptr.as_ptr().add(idx)); }
        }
        
        return None;
    }
}


impl<T: PartialEq> Vector<T> {
    pub fn search(&self, finding: T) -> Option<usize> {
        for i in 0 .. self.length {
            if self.get(i).unwrap() == &finding {
                return Some(i);
            }
        }
        return None;
    }
}


impl<T> Default for Vector<T> {
    fn default() -> Self {
        return Self::new();
    }
}


impl<T> Drop for Vector<T> {
    fn drop(&mut self) {
        unsafe {
            drop_in_place(
        slice_from_raw_parts_mut(self.ptr.as_ptr(), self.length)
            );

            let layout = alloc::Layout::from_size_align_unchecked(
                size_of::<T>() * self.capacity,
                align_of::<T>(),
            );

            alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout)
        }
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

    #[test]
    fn get_integer() {
        let vec: Vector<i32> = vector![1, 2, 3, 4, 5];
        assert_eq!(vec.get(2), Some(&3));
    }

    #[test]
    fn get_float() {
        let vec: Vector<f32> = vector![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(vec.get(2), Some(&3.0));
    }

    #[test]
    fn get_str() {
        let vec: Vector<&str> = vector!["Hey", "You", "should", "get", "ME!"];
        assert_eq!(vec.get(4), Some(&"ME!"));
    }

    #[test]
    fn search_for_integer() {
        let vec: Vector<i32> = vector![1337, 420, 3005, 666, 23];
        assert_eq!(vec.search(666), Some(3));
    }

    #[test]
    fn search_for_float() {
        let vec: Vector<f32> = vector![3.14, 3.60, 5.55, 7.20, 45.0];
        assert_eq!(vec.search(5.55), Some(2));
    }

    #[test]
    fn search_for_str() {
        let vec: Vector<&str> = vector!["Hey", "You", "maybe", "find", "this."];
        assert_eq!(vec.search("this."), Some(4));
    }
}