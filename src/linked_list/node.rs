use std::option::Option;
use std::cmp::PartialEq;
use std::ptr::NonNull;
use std::fmt;


/// `Node` to be used in creating a `LinkedList`.
/// ## Fields:
/// ```rust
/// pub previous: Option<NonNull<Node<T>>> // Previous Node within the LinkedList.
/// pub next: Option<NonNull<Node<T>>> // Next Node within the LinkedList.
/// pub data: T // Node's data.
/// ```
#[derive(Debug)]
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


impl<T: PartialEq> PartialEq for Node<T> {
    fn eq(&self, other: &Self) -> bool {
        return self.data == other.data;
    }
}


#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn create_integer_node() {
        let node = Node::new(0);
        assert_eq!(node, Node::new(0));
    }

    #[test]
    fn create_float_node() {
        let node = Node::new(0.0);
        assert_eq!(node, Node::new(0.0));
    }

    #[test]
    fn create_str_node() {
        let node = Node::new("Zero");
        assert_eq!(node, Node::new("Zero"));
    }

    #[test]
    fn default() {
        let node: Node<u8> = Node::default();
        assert_eq!(node.data, 0);
    }

    #[test]
    fn display() {
        let node = Node::new(1337);
        assert_eq!(format!("{}", node), "1337");
    }

    #[test]
    fn partial_eq() {
        let node = Node::new(500);
        assert_eq!(node, Node::new(500));
        assert_ne!(node, Node::new(0));
    }
}