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
/// list.push(1);
/// list.push(2);
/// list.push(3);
/// assert_eq!(list, list![1, 2, 3]);
/// ```
#[macro_export]
macro_rules! list {
    ($($e:expr), *) => {
        {
            #[allow(unused_mut)]
            let mut list = $crate::linked_list::list::LinkedList::new();
            $(
                list.push($e);
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
pub struct LinkedList<T> {
    pub head: Option<NonNull<Node<T>>>,
    pub tail: Option<NonNull<Node<T>>>,
    length: usize,
}


impl<T> LinkedList<T> {
    pub fn new() -> Self {
        return Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn len(&self) -> usize {
        return self.length;
    }

    pub fn push(&mut self, data: T) {
        let mut new_node: Box<Node<T>> = Box::new(Node::new(data));
        new_node.previous = self.tail;
        new_node.next = None;

        let node_ptr: Option<NonNull<Node<T>>> = Some(
            unsafe { NonNull::new_unchecked(Box::into_raw(new_node)) }
        );

        match self.tail {
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
            None => self.head = node_ptr,
        }

        self.length += 1;
        self.tail = node_ptr;
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        return self.get_node(self.head, index);
    }

    fn get_node(&self, node: Option<NonNull<Node<T>>>, index: usize) -> Option<&T> {
        match node {
            Some(next_ptr) => match index {
                0 => Some( unsafe{ &(*next_ptr.as_ptr()).data } ),
                _ => self.get_node( unsafe { (*next_ptr.as_ptr()).next }, index - 1 ),
            },
            None => None,
        }
    }
}


impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        return Self::new();
    }
}


impl<T: fmt::Debug> fmt::Debug for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.length == 0 { return write!(f, "[]"); }

        let mut ret: String = String::from("[");
        let mut next_node: Option<NonNull<Node<T>>> = self.head;

        while next_node != None {
            unsafe {
                let node_ref: &Node<T> = next_node.unwrap().as_ref();
                ret.push_str(format!("{:?}, ", node_ref).as_str());
                next_node = node_ref.next;
            }
        }

        ret = ret.strip_suffix(", ").unwrap().to_string();
        return write!(f, "{}", ret + "]");
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
}


#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_integer_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);
        list.push(5);
        assert_eq!(list, list![1, 2, 3, 4, 5]);
    }

    #[test]
    fn create_float_list() {
        let mut list: LinkedList<f32> = LinkedList::new();
        list.push(1.0);
        list.push(2.0);
        list.push(3.0);
        list.push(4.0);
        list.push(5.0);
        assert_eq!(list, list![1.0, 2.0, 3.0, 4.0, 5.0]);
    }

    #[test]
    fn create_str_list() {
        let mut list: LinkedList<&str> = LinkedList::new();
        list.push("1");
        list.push("2");
        list.push("3");
        list.push("4");
        list.push("5");
        assert_eq!(list, list!["1", "2", "3", "4", "5"]);
    }

    #[test]
    fn length() {
        assert_eq!(list![1, 2, 3].len(), 3);
    }
}