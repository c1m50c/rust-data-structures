use std::option::Option;
use std::ptr::NonNull;
use std::fmt;


/// `Node` to be used in creating a `LinkedList`.
/// ## Fields:
/// ```rust
/// pub previous: Option<NonNull<Node<T>>> // Previous Node within the LinkedList.
/// pub next: Option<NonNull<Node<T>>> // Next Node within the LinkedList.
/// pub data: T // Node's data.
/// ```
pub struct Node<T> {
    pub previous: Option<NonNull<Node<T>>>,
    pub next: Option<NonNull<Node<T>>>,
    pub data: T,
}


impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        return Self {
            previous: None,
            next: None,
            data,
        }
    }
}


impl<T: Default> Default for Node<T> {
    fn default() -> Self {
        return Self::new(T::default());
    }
}


impl<T: fmt::Display> fmt::Display for Node<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.data);
    }
}