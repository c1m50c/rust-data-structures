#[cfg(test)]
mod tests;

pub(crate) mod node;
use node::Node;

use std::boxed::Box;
use std::vec::Vec;

use core::mem::{swap as mem_swap, replace as mem_replace};
use core::ptr::{NonNull, read as ptr_read};
use core::ops::{Index, IndexMut};
use core::option::Option;
use core::cmp::PartialEq;
use core::str::FromStr;
use core::fmt;


#[allow(unused_macros)]
#[allow(unused_imports)]
pub mod macros {
    /// Macro that is the shorthanded syntax for creating a new `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<u8> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(list, list![1, 2, 3]);
    /// ```
    macro_rules! list {
        ($($e:expr), *) => {
            {
                #[allow(unused_mut)]
                let mut list = $crate::linked_list::LinkedList::new();
                $( list.push_back($e); )*
                list
            }
        };
    }

    pub(crate) use list;
}


/// Rust Implementation of a Doubly Linked List.
/// In modern times this Data Structure is virtually useless, due to the existence of vectors and cache optimizations.
/// ## Fields:
/// ```rust
/// head: Option<NonNull<Node<T>>> // Node at the start of the List.
/// tail: Option<NonNull<Node<T>>> // Node at the end of the List.
/// length: usize // Amount of Nodes within the List.
/// ```
#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    length: usize,
}


/* Private Methods */
impl<T> LinkedList<T> {
    /// Returns a reference to a `Node`'s data value if the `Node` is present at the given index,
    /// and the passed `root` contains the `Node` at a given `next` reference.
    #[inline]
    fn get_node(&self, root: Option<NonNull<Node<T>>>, index: usize) -> Option<&Node<T>> {
        match root {
            Some(ptr) => match index {
                0 => Some( unsafe{ &(*ptr.as_ptr()) } ),
                _ => self.get_node( unsafe { (*ptr.as_ptr()).next }, index - 1 ),
            },
            
            None => None,
        }
    }

    /// Returns a mutable reference to a `Node`'s data value if the `Node` is present at the given index,
    /// and the passed `root` contains the `Node` at a given `next` reference.
    #[inline]
    fn get_node_mut(&self, root: Option<NonNull<Node<T>>>, index: usize) -> Option<&mut Node<T>> {
        match root {
            Some(ptr) => match index {
                0 => Some( unsafe{ &mut (*ptr.as_ptr()) } ),
                _ => self.get_node_mut( unsafe { (*ptr.as_ptr()).next }, index - 1 ),
            },

            None => None,
        }
    }
}


/* Public Methods */
impl<T> LinkedList<T> {
    /// Constructs a new empty `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<f64> = LinkedList::new();
    /// list.push_back(13.37);
    /// list.push_back(4.04);
    /// list.push_back(2.00);
    /// assert_eq!(list, list![13.37, 4.04, 2.00]);
    /// ```
    #[inline(always)]
    pub const fn new() -> Self {
        return Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    /// Returns the `length` of the `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let list: LinkedList<&str> = list!["This", "is", "a", "Linked", "List"];
    /// assert_eq!(list.len(), 5);
    /// ```
    #[inline(always)]
    pub const fn len(&self) -> usize {
        return self.length;
    }

    /// Returns a `bool` that determines if the list is empty.
    /// ## Example:
    /// ```rust
    /// let list: LinkedList<i32> = list![1, 2, 3];
    /// assert_eq!(list.is_empty(), false);
    /// list.clear();
    /// assert_eq!(list.is_empty(), true);
    /// ```
    #[inline(always)]
    pub const fn is_empty(&self) -> bool {
        return self.head.is_none();
    }

    /// Clears the `LinkedList`, making it completely empty and resetting its `length`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<&str> = list!["Hey", "how", "it", "b"];
    /// assert_eq!(list, list!["Hey", "how", "it", "b"]);
    /// list.clear()
    /// assert_eq!(list, LinkedList::<&str>::new());
    /// ```
    #[inline(always)]
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Pushes or prepends a new `Node` to the start of the `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_front(1);
    /// list.push_front(2);
    /// list.push_front(3);
    /// assert_eq!(list, list![3, 2, 1]);
    /// ```
    #[inline]
    pub fn push_front(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next = self.head;
        new_node.previous = None;

        let node_ptr = unsafe { Some(NonNull::new_unchecked(Box::into_raw(new_node))) };

        match self.head {
            Some(ptr) => unsafe { (*ptr.as_ptr()).previous = node_ptr },
            None => self.tail = node_ptr,
        }

        self.length += 1;
        self.head = node_ptr;
    }

    /// Pushes or appends a new `Node` to the end of the `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<i32> = LinkedList::new();
    /// list.push_back(1);
    /// list.push_back(2);
    /// list.push_back(3);
    /// assert_eq!(list, list![1, 2, 3]);
    /// ```
    #[inline]
    pub fn push_back(&mut self, data: T) {
        let mut new_node = Box::new(Node::new(data));
        new_node.previous = self.tail;
        new_node.next = None;

        let node_ptr = unsafe { Some(NonNull::new_unchecked(Box::into_raw(new_node))) };

        match self.tail {
            Some(ptr) => unsafe { (*ptr.as_ptr()).next = node_ptr },
            None => self.head = node_ptr,
        }

        self.length += 1;
        self.tail = node_ptr;
    }

    /// Inserts a new `Node` into the `LinkedList` at the given index.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<u64> = list![1, 3];
    /// list.insert(3, 1);
    /// assert_eq!(list, list![1, 2, 3]);
    /// assert_eq!(list.len(), 3);
    /// assert_eq!(list[1], 3);
    /// ```
    pub fn insert(&mut self, data: T, index: usize) {
        let mut current = self.head;
        let mut i = 0;

        while let Some(mut ptr) = current {
            if index == i {
                let mut new_node = Box::new(Node::new(data));

                new_node.previous = unsafe { ptr.as_ref().previous };
                new_node.next = Some(ptr);

                unsafe {
                    let mut node_ptr = NonNull::new_unchecked(Box::into_raw(new_node));
                    ptr.as_mut().previous = Some(node_ptr);
                    
                    // TODO: 🧼 Probs some cleanup potential here, unwrapping is a bit sloppy.
                    if let Some(mut ptr) = node_ptr.as_mut().previous {
                        ptr.as_mut().next = Some(node_ptr);
                    }
                }

                self.length += 1;
                return;
            }

            i += 1;
            current = unsafe { ptr.as_ref().next };
        }
    }

    /// Removes the first `Node` within the `LinkedList` and returns a reference to its `data` field.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<i32> = list![1, 2, 3, 4, 5];
    /// let pop: Option<i32> = list.pop_front();
    /// assert_eq!(pop, Some(1));
    /// assert_eq!(list, list![2, 3, 4, 5]);
    /// ```
    #[inline]
    pub fn pop_front(&mut self) -> Option<T> {
        if let Some(mut ptr) = self.head {
            let value: T;
            
            unsafe {
                value = ptr_read(&mut (*ptr.as_mut()).data);
                self.head = (*self.head.unwrap().as_ptr()).next;
                if let Some(ptr) = self.head {
                    (*ptr.as_ptr()).previous = None;
                }
            }
            
            self.length -= 1;
            return Some(value);
        }

        return None;
    }

    /// Removes the last `Node` within the `LinkedList` and returns a reference to its `data` field.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<i32> = list![1, 2, 3, 4, 5];
    /// let pop: Option<i32> = list.pop_back();
    /// assert_eq!(pop, Some(5));
    /// assert_eq!(list, list![1, 2, 3, 4]);
    /// ```
    #[inline]
    pub fn pop_back(&mut self) -> Option<T> {
        if let Some(mut ptr) = self.tail {
            let value: T;
            
            unsafe {
                value = ptr_read(&mut (*ptr.as_mut()).data);
                self.tail = (*self.tail.unwrap().as_ptr()).previous;
                if let Some(ptr) = self.tail {
                    (*ptr.as_ptr()).next = None;
                }
            }
            
            self.length -= 1;
            return Some(value);
        }

        return None;
    }

    /// Removes the first `Node` within the `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<&str> = list!["THIS", "will", "be", "removed."];
    /// list.remove_front();
    /// assert_eq!(list, list!["will", "be", "removed."]);
    /// ```
    #[inline]
    pub fn remove_front(&mut self) {
        if let Some(ptr) = self.head {
            unsafe {
                self.head = (*ptr.as_ptr()).next;
                if let Some(ptr) = self.head {
                    (*ptr.as_ptr()).previous = None;
                }
            }

            self.length -= 1;
        }
    }

    /// Removes the last `Node` within the `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<&str> = list!["Please", "don't", "remove", "ME!"];
    /// list.remove_back();
    /// assert_eq!(list, list!["Please", "don't", "remove"]);
    /// ```
    #[inline]
    pub fn remove_back(&mut self) {
        if let Some(ptr) = self.tail {
            unsafe {
                self.tail = (*ptr.as_ptr()).previous;
                if let Some(ptr) = self.tail {
                    (*ptr.as_ptr()).next = None;
                }
            }

            self.length -= 1;
        }

        
    }

    /// Appends another list to the end of the list.
    /// ## Example:
    /// ```rust
    /// let mut list_one: LinkedList<i32> = list![1, 2, 3];
    /// let mut list_two: LinkedList<i32> = list![4, 5, 6];
    /// list_one.append_list(&mut list_two);
    /// assert_eq!(list_one, list![1, 2, 3, 4, 5, 6]);
    /// ```
    #[inline]
    pub fn append_list(&mut self, other: &mut Self) {
        match self.tail {
            Some(mut self_ptr) => {
                if let Some(mut other_ptr) = other.head.take() {
                    unsafe {
                        self_ptr.as_mut().next = Some(other_ptr);
                        other_ptr.as_mut().previous = Some(self_ptr);
                    }

                    self.tail = other.tail.take();
                    self.length += mem_replace(&mut other.length, 0);
                }
            },

            None => { mem_swap(self, other); },
        }
    }

    /// Returns a reference to a `Node`'s data value if the `Node` is present at the given index.
    /// ## Example:
    /// ```rust
    /// let list: LinkedList<&str> = list!["Get", "This"];
    /// assert_eq!(list.get(1), Some(&"This"));
    /// ```
    #[inline(always)]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index == 0 { return self.front(); }
        else if index == self.length - 1 { return self.back(); }

        match self.get_node(self.head, index) {
            Some(node) => Some(&node.data),
            None => None,
        }
    }

    /// Returns a mutable reference to a `Node`'s data value if the `Node` is present at the given index.
    /// ## Example:
    /// ```rust
    /// let list: LinkedList<&str> = list!["Get", "This"];
    /// assert_eq!(list.get_mut(1), Some(&mut "This"));
    /// ```
    #[inline(always)]
    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        if index == 0 { return self.front_mut(); }
        else if index == self.length - 1 { return self.back_mut(); }
        
        match self.get_node_mut(self.head, index) {
            Some(node) => Some(&mut node.data),
            None => None,
        }
    }

    /// Returns a reference to the `Node` at the front of the list.
    #[inline(always)]
    pub fn front(&self) -> Option<&T> {
        match self.head {
            Some(ptr) => unsafe { Some(&ptr.as_ref().data) },
            None => None,
        }
    }

    /// Returns a mutable reference to the `Node` at the front of the list.
    #[inline(always)]
    pub fn front_mut(&self) -> Option<&mut T> {
        match self.head {
            Some(mut ptr) => unsafe { Some(&mut ptr.as_mut().data) },
            None => None,
        }
    }

    /// Returns a reference to the `Node` at the back of the list.
    #[inline(always)]
    pub fn back(&self) -> Option<&T> {
        match self.tail {
            Some(ptr) => unsafe { Some(&ptr.as_ref().data) },
            None => None,
        }
    }

    /// Returns a mutable reference to the `Node` at the back of the list.
    #[inline(always)]
    pub fn back_mut(&self) -> Option<&mut T> {
        match self.tail {
            Some(mut ptr) => unsafe { Some(&mut ptr.as_mut().data) },
            None => None,
        }
    }

    /// Returns the `LinkedList` as a `Vec`.
    /// ## Example:
    /// ```rust
    /// let list = list![1, 3, 3, 7];
    /// assert_eq!(list.as_vector(), vec![1, 3, 3, 7]);
    /// ```
    #[inline]
    pub fn as_vector(&self) -> Vec<T> {
        let mut vector = Vec::with_capacity(self.length);
        let mut current = self.head;

        while let Some(ptr) = current {
            let value;

            unsafe {
                value = ptr_read(&(*ptr.as_ref()).data);
                current = ptr.as_ref().next;
            }

            vector.push(value);
        }
        
        return vector;
    }
}


impl<T: PartialEq> LinkedList<T> {
    /// Searches through the `LinkedList` for a `Node` that contains the equivalent value of `finding`,
    /// returning its index if found.
    /// ## Example:
    /// ```rust
    /// let list: LinkedList<&str> = list!["Hey", "find", "THIS!"];
    /// assert_eq!(list.search("THIS!"), Some(2));
    /// ```
    #[inline]
    pub fn search(&self, finding: T) -> Option<usize> {
        let mut current = self.head;
        let mut index = 0;

        while let Some(ptr) = current {
            let node_ref = unsafe { ptr.as_ref() };
            if node_ref.data == finding { return Some(index); }
            current = node_ref.next;
            index += 1;
        }

        return None;
    }
}


impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        return Self::new();
    }
}


impl<T: fmt::Display> fmt::Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.length == 0 { return write!(f, "[]"); }

        let mut result = String::from("[");
        let mut current = self.head;

        while let Some(ptr) = current {
            let ptr_ref = unsafe { ptr.as_ref() };
            result.push_str(format!("{}, ", ptr_ref).as_str());
            current = ptr_ref.next;
        }

        return write!(f, "{}", result.strip_suffix(", ").unwrap().to_string() + "]");
    }
}


impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() { return false; }
        if self.len() == 0 { return true; }

        let (mut s, mut o) = (self.head, other.head);

        while let (Some(s_ptr), Some(o_ptr)) = (s, o) {
            unsafe {
                if s_ptr.as_ref().data != o_ptr.as_ref().data { return false; }
                s = (*s_ptr.as_ptr()).next;
                o = (*o_ptr.as_ptr()).next;
            }
        }

        return true;
    }
}


impl<T: Eq> Eq for LinkedList<T> {  }


impl<T> Index<usize> for LinkedList<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        return self.get(index).unwrap().to_owned();
    }
}


impl<T> IndexMut<usize> for LinkedList<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return self.get_mut(index).unwrap();
    }
}


impl<T: Copy> From<Vec<T>> for LinkedList<T> {
    fn from(vec: Vec<T>) -> Self {
        let mut list: LinkedList<T> = LinkedList::new();
        for i in 0 .. vec.len() { list.push_back(vec[i]); }
        return list;
    }
}


impl<T: Copy> From<&[T]> for LinkedList<T> {
    fn from(slice: &[T]) -> Self {
        let mut list: LinkedList<T> = LinkedList::new();
        for i in 0 .. slice.len() { list.push_back(slice[i]); }
        return list;
    }
}


impl<T: Copy, const N: usize> From<[T; N]> for LinkedList<T> {
    fn from(arr: [T; N]) -> Self {
        let mut list: LinkedList<T> = LinkedList::new();
        for i in 0 .. N { list.push_back(arr[i]); }
        return list;
    }
}


impl<T: FromStr> FromStr for LinkedList<T> {
    type Err = T::Err;
    
    /*
        NOTE: Formatting for correct parsing is not the same as LinkedList's `display` trait output.
        "5 4 3 2 1".parse::<LinkedList<i32>>().unwrap() == list![5, 4, 3, 2, 1]
    */
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result: LinkedList<T> = LinkedList::new();
        let split = s.split(" ");

        for s in split {
            let x = s.parse::<T>()?;
            result.push_back(x);
        }

        return Ok(result);
    }
}