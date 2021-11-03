use std::slice::from_raw_parts_mut as slice_from_raw_parts_mut;
use std::ptr::{NonNull, drop_in_place};
use std::mem::{size_of, align_of};
use std::ops::{Index, IndexMut};
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


/// Mimic implementation of `std::vec::Vec`.
/// ## Fields:
/// ```rust
/// ptr: NonNull<T> // Pointer to the Vector in memory.
/// capacity: usize // Capacity of the Vector.
/// length: usize // Length of the Vector.
/// ```
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

    /// Pushs a new value into the `Vector`
    /// ## Example:
    /// ```rust
    /// let mut vector: Vector<i32> = Vector::new();
    /// vector.push(4);
    /// vector.push(0);
    /// vector.push(4);
    /// assert_eq!(vector, vector![4, 0, 4]);
    /// ```
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

    /// Returns a reference to the value at the given index if it exists.
    pub fn get(&self, idx: usize) -> Option<&T> {
        if idx < self.length {
            unsafe { return Some(&*self.ptr.as_ptr().add(idx)); }
        }
        
        return None;
    }

    /// Returns a mutable reference to the value at the given index if it exists.
    pub fn get_mut(&self, idx: usize) -> Option<&mut T> {
        if idx < self.length {
            unsafe { return Some(&mut *self.ptr.as_ptr().add(idx)); }
        }
        
        return None;
    }
}


impl<T: PartialEq> Vector<T> {
    /// Searches through the `Vector` to find a value that matches `finding`, returning its index if found.
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


impl<T: PartialEq> PartialEq for Vector<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.length != other.len() { return false; }

        for i in 0 .. self.length {
            let (sget, oget) = (self.get(i), other.get(i));
            if sget.is_some() != sget.is_some() { return false; }
            if sget.is_some() {
                if sget.unwrap() != oget.unwrap() { return false; }
            }
        }

        return true;
    }
}


impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index).unwrap();
    }
}


impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.get_mut(index).unwrap();
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
    fn get_mut_integer() {
        let vec: Vector<i32> = vector![1, 2, 3, 4, 5];
        let got = vec.get_mut(2).unwrap();
        assert_eq!(vec.get_mut(2), Some(&mut 3));
        *got = 72;
        assert_eq!(vec.get_mut(2), Some(&mut 72));
    }

    #[test]
    fn get_mut_float() {
        let vec: Vector<f32> = vector![1.0, 2.0, 3.0, 4.0, 5.0];
        let got = vec.get_mut(2).unwrap();
        assert_eq!(vec.get_mut(2), Some(&mut 3.0));
        *got = 72.0;
        assert_eq!(vec.get_mut(2), Some(&mut 72.0));
    }

    #[test]
    fn get_mut_str() {
        let vec: Vector<&str> = vector!["Hey", "You", "should", "get", "ME!"];
        let got = vec.get_mut(4).unwrap();
        assert_eq!(vec.get_mut(4), Some(&mut "ME!"));
        *got = "ME! But mutable..";
        assert_eq!(vec.get_mut(4), Some(&mut "ME! But mutable.."));
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

    #[test]
    fn partial_eq() {
        let vec_one: Vector<i32> = vector![5, 0, 5, 7, 8, 8];
        let vec_two: Vector<i32> = vector![5, 0, 5, 7, 8, 8];
        assert_eq!(vec_one, vec_two);
        assert_ne!(vec_one, vector![0, 3, 0, 0, 0, 0]);
    }

    #[test]
    fn index() {
        let vec: Vector<&str> = vector!["Hey", "this", "is", "a", "Vector"];
        assert_eq!(vec[0], "Hey");
        assert_eq!(vec[1], "this");
        assert_eq!(vec[2], "is");
        assert_eq!(vec[3], "a");
        assert_eq!(vec[4], "Vector");
    }

    #[test]
    fn index_mut() {
        let mut vec: Vector<i32> = vector![3, 3, 3];
        assert_eq!(vec, vector![3, 3, 3]);
        vec[0] = 6;
        vec[1] = 6;
        vec[2] = 6;
        assert_eq!(vec, vector![6, 6, 6]);
    }
}