use super::node::Node;

use std::option::Option;
use std::cmp::PartialEq;
use std::ptr::NonNull;
use std::boxed::Box;
use std::fmt;


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
            let mut list = $crate::linked_list::list::LinkedList::new();
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
/// pub head: Option<NonNull<Node<T>>> // Node at the start of the List.
/// pub tail: Option<NonNull<Node<T>>> // Node at the end of the List.
/// length: usize // Amount of Nodes within the List.
/// ```
#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    length: usize,
}


/* Private Methods */
impl<T> LinkedList<T> {
    /// Returns a reference to a `Node`'s data value if the `Node` is present at the given index,
    /// and the passed `root` contains the `Node` at a given `next` reference.
    fn get_node(&self, root: Option<NonNull<Node<T>>>, index: usize) -> Option<&T> {
        match root {
            Some(next_ptr) => match index {
                0 => Some( unsafe{ &(*next_ptr.as_ptr()).data } ),
                _ => self.get_node( unsafe { (*next_ptr.as_ptr()).next }, index - 1 ),
            },
            None => None,
        }
    }
}


/* Public Methods */
impl<T> LinkedList<T> {
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
    pub fn len(&self) -> usize {
        return self.length;
    }

    /// Clears the `LinkedList`, making it completely empty and resetting its `length`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<&str> = list!["Hey", "how", "it", "b"];
    /// assert_eq!(list, list!["Hey", "how", "it", "b"]);
    /// list.clear()
    /// assert_eq!(list, LinkedList::<&str>::new());
    /// ```
    pub fn clear(&mut self) {
        *self = Self::new();
    }

    /// Returns a `bool` that determines if the list is empty.
    /// ## Example:
    /// ```rust
    /// let list: LinkedList<i32> = list![1, 2, 3];
    /// assert_eq!(list.is_empty(), false);
    /// list.clear();
    /// assert_eq!(list.is_empty(), true);
    /// ```
    pub fn is_empty(&self) -> bool {
        return self.head.is_none();
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
    pub fn push_front(&mut self, data: T) {
        let mut new_node: Box<Node<T>> = Box::new(Node::new(data));
        new_node.next = self.head;
        new_node.previous = None;

        let node_ptr = unsafe { Some(NonNull::new_unchecked(Box::into_raw(new_node))) };

        match self.head {
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).previous = node_ptr },
            None => self.tail = node_ptr,
        }

        self.head = node_ptr;
        self.length += 1;
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

    /// Removes the last `Node` within the `LinkedList`.
    /// ## Example:
    /// ```rust
    /// let mut list: LinkedList<&str> = list!["Please", "don't", "remove", "ME!"];
    /// assert_eq!(list, list!["Please", "don't", "remove", "ME!"]);
    /// list.remove_back();
    /// assert_eq!(list, list!["Please", "don't", "remove"]);
    /// ```
    pub fn remove_back(&mut self) {
        if self.tail != None {
            unsafe {
                self.tail = (*self.tail.unwrap().as_ptr()).previous;
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
    pub fn get(&self, index: usize) -> Option<&T> {
        return self.get_node(self.head, index);
    }

    /// Returns a reference to the `Node` at the front of the list.
    pub fn front(&self) -> Option<&T> {
        if self.head.is_none() { return None; }
        
        unsafe {
            return Some(&self.head.unwrap().as_ref().data);
        }
    }

    /// Returns a mutable reference to the `Node` at the front of the list.
    pub fn front_mut(&self) -> Option<&mut T> {
        if self.head.is_none() { return None; }
        
        unsafe {
            return Some(&mut self.head.unwrap().as_mut().data);
        }
    }

    /// Returns a reference to the `Node` at the back of the list.
    pub fn back(&self) -> Option<&T> {
        if self.tail.is_none() { return None; }
        
        unsafe {
            return Some(&self.tail.unwrap().as_ref().data);
        }
    }

    /// Returns a mutable reference to the `Node` at the back of the list.
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
    fn push_front_integer() {
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
    fn push_back_integer() {
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
    fn push_front_float() {
        let mut list: LinkedList<f32> = LinkedList::new();
        list.push_front(1.0);
        list.push_front(2.0);
        list.push_front(3.0);
        list.push_front(4.0);
        list.push_front(5.0);
        assert_eq!(list, list![5.0, 4.0, 3.0, 2.0, 1.0]);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn pust_back_float() {
        let mut list: LinkedList<f32> = LinkedList::new();
        list.push_back(1.0);
        list.push_back(2.0);
        list.push_back(3.0);
        list.push_back(4.0);
        list.push_back(5.0);
        assert_eq!(list, list![1.0, 2.0, 3.0, 4.0, 5.0]);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn push_front_str() {
        let mut list: LinkedList<&str> = LinkedList::new();
        list.push_front("1");
        list.push_front("2");
        list.push_front("3");
        list.push_front("4");
        list.push_front("5");
        assert_eq!(list, list!["5", "4", "3", "2", "1"]);
        assert_eq!(list.len(), 5);
    }

    #[test]
    fn push_back_str() {
        let mut list: LinkedList<&str> = LinkedList::new();
        list.push_back("1");
        list.push_back("2");
        list.push_back("3");
        list.push_back("4");
        list.push_back("5");
        assert_eq!(list, list!["1", "2", "3", "4", "5"]);
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
    fn remove_back_integer() {
        let mut list: LinkedList<i32> = list![1, 2, 3];
        list.remove_back();
        assert_eq!(list, list![1, 2]);
    }

    #[test]
    fn remove_back_float() {
        let mut list: LinkedList<f32> = list![1.0, 2.0, 3.0];
        list.remove_back();
        assert_eq!(list, list![1.0, 2.0]);
    }

    #[test]
    fn remove_back_str() {
        let mut list: LinkedList<&str> = list!["One", "Two", "Three"];
        list.remove_back();
        assert_eq!(list, list!["One", "Two"]);
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
    fn get_integer() {
        let list: LinkedList<i32> = list![1, 2, 3, 4, 5];
        assert_eq!(list.get(2), Some(&3));
    }

    #[test]
    fn get_float() {
        let list: LinkedList<f32> = list![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(list.get(2), Some(&3.0));
    }
    
    #[test]
    fn get_str() {
        let list: LinkedList<&str> = list!["Get", "This", "Ok?"];
        assert_eq!(list.get(1), Some(&"This"));
    }

    #[test]
    fn search_for_integer() {
        let list: LinkedList<i32> = list![1, 2, 3, 4, 5];
        assert_eq!(list.search(3), Some(2));
    }

    #[test]
    fn search_for_float() {
        let list: LinkedList<f32> = list![1.0, 2.0, 3.0, 4.0, 5.0];
        assert_eq!(list.search(3.0), Some(2));
    }

    #[test]
    fn search_for_str() {
        let list: LinkedList<&str> = list!["Search", "Idk", "Maybe", "This?"];
        assert_eq!(list.search("This?"), Some(3));
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