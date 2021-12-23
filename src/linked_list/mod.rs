pub(crate) mod node;

use node::Node;

use std::boxed::Box;

use core::ptr::{NonNull, read as ptr_read};
use core::ops::{Index, IndexMut};
use core::option::Option;
use core::cmp::PartialEq;
use core::fmt;


/// Shorthand Syntax for creating a new `LinkedList`.
/// ## Example:
/// ```rust
/// let mut list: LinkedList<u8> = LinkedList::new();
/// list.push_back(1);
/// list.push_back(2);
/// list.push_back(3);
/// assert_eq!(list, list![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! list {
    ($($e:expr), *) => {
        {
            #[allow(unused_mut)]
            let mut list = $crate::linked_list::LinkedList::new();
            $(
                list.push_back($e);
            )*
            list
        }
    };
}


/// Implementation of a Doubly Linked List.
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
    fn get_node(&self, root: Option<NonNull<Node<T>>>, index: usize) -> Option<&T> {
        match root {
            Some(next_ptr) => match index {
                0 => Some( unsafe{ &(*next_ptr.as_ptr()).data } ),
                _ => self.get_node( unsafe { (*next_ptr.as_ptr()).next }, index - 1 ),
            },
            
            None => None,
        }
    }

    /// Returns a mutable reference to a `Node`'s data value if the `Node` is present at the given index,
    /// and the passed `root` contains the `Node` at a given `next` reference.
    #[inline]
    fn get_node_mut(&self, root: Option<NonNull<Node<T>>>, index: usize) -> Option<&mut T> {
        match root {
            Some(next_ptr) => match index {
                0 => Some( unsafe{ &mut (*next_ptr.as_ptr()).data } ),
                _ => self.get_node_mut( unsafe { (*next_ptr.as_ptr()).next }, index - 1 ),
            },

            None => None,
        }
    }
}


/* Public Methods */
impl<T> LinkedList<T> {
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
        let mut new_node: Box<Node<T>> = Box::new(Node::new(data));
        new_node.next = self.head;
        new_node.previous = None;

        let node_ptr = unsafe { Some(NonNull::new_unchecked(Box::into_raw(new_node))) };

        match self.head {
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).previous = node_ptr },
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
        let mut new_node: Box<Node<T>> = Box::new(Node::new(data));
        new_node.previous = self.tail;
        new_node.next = None;

        let node_ptr: Option<NonNull<Node<T>>> = unsafe { Some(NonNull::new_unchecked(Box::into_raw(new_node))) };

        match self.tail {
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
            None => self.head = node_ptr,
        }

        self.length += 1;
        self.tail = node_ptr;
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
        if self.head.is_some() {
            let value: T;
            
            unsafe {
                value = ptr_read(&mut (*self.head.unwrap().as_mut()).data);
                self.head = (*self.head.unwrap().as_ptr()).next;
                if self.head.is_some() { (*self.head.unwrap().as_ptr()).previous = None; }
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
        if self.tail.is_some() {
            let value: T;
            
            unsafe {
                value = ptr_read(&mut (*self.tail.unwrap().as_mut()).data);
                self.tail = (*self.tail.unwrap().as_ptr()).previous;
                if self.tail.is_some() { (*self.tail.unwrap().as_ptr()).next = None; }
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
        if self.head.is_some() {
            unsafe {
                self.head = (*self.head.unwrap().as_ptr()).next;
                if self.head.is_some() { (*self.head.unwrap().as_ptr()).previous = None; }
                self.length -= 1;
            }
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
        if self.tail.is_some() {
            unsafe {
                self.tail = (*self.tail.unwrap().as_ptr()).previous;
                if self.tail.is_some() { (*self.tail.unwrap().as_ptr()).next = None; }
                self.length -= 1;
            }
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
            None => std::mem::swap(self, other),
            Some(mut stail_ptr) => {
                if let Some(mut ohead_ptr) = other.head.take() {
                    unsafe {
                        stail_ptr.as_mut().next = Some(ohead_ptr);
                        ohead_ptr.as_mut().previous = Some(stail_ptr);
                    }

                    self.tail = other.tail.take();
                    self.length += std::mem::replace(&mut other.length, 0);
                }
            }
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
        return self.get_node(self.head, index);
    }

    /// Returns a mutable reference to a `Node`'s data value if the `Node` is present at the given index.
    /// ## Example:
    /// ```rust
    /// let list: LinkedList<&str> = list!["Get", "This"];
    /// assert_eq!(list.get_mut(1), Some(&mut "This"));
    /// ```
    #[inline(always)]
    pub fn get_mut(&self, index: usize) -> Option<&mut T> {
        return self.get_node_mut(self.head, index);
    }

    /// Returns a reference to the `Node` at the front of the list.
    #[inline(always)]
    pub fn front(&self) -> Option<&T> {
        if self.head.is_none() { return None; }
        
        unsafe {
            return Some(&self.head.unwrap().as_ref().data);
        }
    }

    /// Returns a mutable reference to the `Node` at the front of the list.
    #[inline(always)]
    pub fn front_mut(&self) -> Option<&mut T> {
        if self.head.is_none() { return None; }
        
        unsafe {
            return Some(&mut self.head.unwrap().as_mut().data);
        }
    }

    /// Returns a reference to the `Node` at the back of the list.
    #[inline(always)]
    pub fn back(&self) -> Option<&T> {
        if self.tail.is_none() { return None; }
        
        unsafe {
            return Some(&self.tail.unwrap().as_ref().data);
        }
    }

    /// Returns a mutable reference to the `Node` at the back of the list.
    #[inline(always)]
    pub fn back_mut(&self) -> Option<&mut T> {
        if self.tail.is_none() { return None; }
        
        unsafe {
            return Some(&mut self.tail.unwrap().as_mut().data);
        }
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
        let mut next_node = self.head;

        for i in 0 .. self.length {
            unsafe {
                let node_ref = next_node.unwrap().as_ref();
                if node_ref.data == finding { return Some(i); }
                next_node = next_node.unwrap().as_ref().next;
            }
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

        let mut ret: String = String::from("[");
        let mut next_node: Option<NonNull<Node<T>>> = self.head;

        while next_node != None {
            unsafe {
                let node_ref: &Node<T> = next_node.unwrap().as_ref();
                ret.push_str(format!("{}, ", node_ref).as_str());
                next_node = node_ref.next;
            }
        }

        ret = ret.strip_suffix(", ").unwrap().to_string();
        return write!(f, "{}", ret + "]");
    }
}


impl<T: PartialEq> PartialEq for LinkedList<T> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() { return false; }

        for i in 0 .. self.len() {
            let (sget, oget) = (self.get(i), other.get(i));
            if sget.is_some() != oget.is_some() { return false; }
            if sget.is_some() {
                if sget.unwrap() != oget.unwrap() { return false; }
            }
        }

        return true;
    }

    fn ne(&self, other: &Self) -> bool {
        if self.len() != other.len() { return true; }

        for i in 0 .. self.len() {
            let (sget, oget) = (self.get(i), other.get(i));
            if sget.is_some() != oget.is_some() { return true; }
            if sget.is_some() {
                if sget.unwrap() != oget.unwrap() { return true; }
            }
        }

        return false;
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


impl<T: Copy> From<std::vec::Vec<T>> for LinkedList<T> {
    fn from(vec: std::vec::Vec<T>) -> Self {
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


#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn push_front() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        list.push_front(4);
        list.push_front(5);
        assert_eq!(list, list![5, 4, 3, 2, 1]);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn push_back() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        list.push_back(4);
        list.push_back(5);
        assert_eq!(list, list![1, 2, 3, 4, 5]);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn append_list() {
        let mut list_one: LinkedList<&str> = list!["One", "Two", "Three"];
        let mut list_two: LinkedList<&str> = list!["Four", "Five", "Six"];
        list_one.append_list(&mut list_two);
        assert_eq!(list_one, list!["One", "Two", "Three", "Four", "Five", "Six"]);
        assert_eq!(list_one.len(), 6);
    }

    #[test]
    fn remove() {
        let mut list: LinkedList<&str> = list!["One", "Two", "Three"];
        list.remove_back();
        assert_eq!(list, list!["One", "Two"]);
        list.remove_front();
        assert_eq!(list, list!["Two"]);
    }

    #[test]
    fn pop() {
        let mut list: LinkedList<i32> = list![1, 2, 3, 4, 5];
        
        let pop_front = list.pop_front();
        assert_eq!(pop_front, Some(1));

        let pop_back = list.pop_back();
        assert_eq!(pop_back, Some(5));
        
        assert_eq!(list, list![2, 3, 4]);
    }

    #[test]
    fn clear() {
        let mut list: LinkedList<i32> = list![6, 6, 6];
        list.clear();
        assert_eq!(list, LinkedList::<i32>::new());
    }

    #[test]
    fn is_empty() {
        let mut list: LinkedList<&str> = list!["I", "am", "not", "empty."];
        assert_eq!(list.is_empty(), false);
        list.clear();
        assert_eq!(list.is_empty(), true);
    }

    #[test]
    fn get() {
        let list: LinkedList<i32> = list![1, 2, 3, 4, 5];
        assert_eq!(list.get(2), Some(&3));
    }

    #[test]
    fn get_mut() {
        let list: LinkedList<f32> = list![1.0, 2.0, 3.0, 4.0, 5.0];
        let got = list.get_mut(2).unwrap();
        assert_eq!(list.get_mut(2), Some(&mut 3.0));
        *got = 6.0;
        assert_eq!(list.get_mut(2), Some(&mut 6.0));
    }

    #[test]
    fn search() {
        let list: LinkedList<&str> = list!["Search", "Idk", "Maybe", "This?"];
        assert_eq!(list.search("This?"), Some(3));
    }

    #[test]
    fn index() {
        let list: LinkedList<&str> = list!["Hey", "this", "is", "a", "Linked", "List"];
        assert_eq!(list[0], "Hey");
        assert_eq!(list[1], "this");
        assert_eq!(list[2], "is");
        assert_eq!(list[3], "a");
        assert_eq!(list[4], "Linked");
        assert_eq!(list[5], "List");
    }

    #[test]
    fn index_mut() {
        let mut list: LinkedList<i32> = list![3, 3, 3];
        assert_eq!(list, list![3, 3, 3]);
        list[0] = 6;
        list[1] = 6;
        list[2] = 6;
        assert_eq!(list, list![6, 6, 6]);
    }

    #[test]
    fn default() {
        let list: LinkedList<u8> = LinkedList::default();
        assert_eq!(list, LinkedList::new());
    }

    #[test]
    fn length() {
        assert_eq!(list![1, 2, 3].len(), 3);
    }

    #[test]
    fn display() {
        assert_eq!(format!("{}", list![1, 3, 3, 7]), "[1, 3, 3, 7]");
        assert_eq!(format!("{}", LinkedList::<i32>::new()), "[]");
    }

    #[test]
    fn from_vec() {
        let list = LinkedList::from(vec![1, 2, 3]);
        assert_eq!(list, list![1, 2, 3]);
    }

    #[test]
    fn from_slice() {
        let slice: &[i32] = &[1, 2, 3];
        let list = LinkedList::from(slice);
        assert_eq!(list, list![1, 2, 3]);
    }

    #[test]
    fn from_array() {
        let arr: [i32; 3] = [1, 2, 3];
        let list = LinkedList::from(arr);
        assert_eq!(list, list![1, 2, 3]);
    }
}