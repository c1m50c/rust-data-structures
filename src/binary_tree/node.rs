use std::option::Option;
use std::ptr::NonNull;
use std::fmt;


/// Struct for creating the `Node`s of a `BinaryTree`.
/// ## Fields:
/// ```rust
/// pub left: Option<NonNull<Node<T>>> // Left child Node within the Tree.
/// pub right: Option<NonNull<Node<T>>> // Right child Node within the Tree.
/// pub data: T // Data of the Node.
/// ```
#[derive(Debug)]
pub struct Node<T> {
    pub left: Option<NonNull<Node<T>>>,
    pub right: Option<NonNull<Node<T>>>,
    pub data: T
}


impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        return Self {
            left: None,
            right: None,
            data,
        };
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


#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn create_integer_node() {
        let node: Node<i32> = Node::new(0);
        assert_eq!(node.data, 0);
    }

    #[test]
    fn create_float_node() {
        let node: Node<f32> = Node::new(0.0);
        assert_eq!(node.data, 0.0);
    }

    #[test]
    fn create_str_node() {
        let node: Node<&str> = Node::new("BinaryTree Node");
        assert_eq!(node.data, "BinaryTree Node");
    }

    #[test]
    fn default() {
        let node: Node<u8> = Node::default();
        assert_eq!(node.data, 0);
    }

    #[test]
    fn display() {
        let node: Node<i32> = Node::new(777);
        assert_eq!(format!("{}", node), "777");
    }
}