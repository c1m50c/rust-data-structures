use std::option::Option;
use std::ptr::NonNull;
use std::fmt;


/// Struct for creating the `Node`s of a `BinaryTree`.
/// ## Fields:
/// ```rust
/// left: Option<NonNull<Node<T>>> // Left child Node within the Tree.
/// right: Option<NonNull<Node<T>>> // Right child Node within the Tree.
/// data: T // Data of the Node.
/// ```
#[derive(Debug)]
pub struct Node<T> {
    left: Option<NonNull<Node<T>>>,
    right: Option<NonNull<Node<T>>>,
    data: T,
}


impl<T> Node<T> {
    pub const fn new(data: T) -> Self {
        return Self {
            left: None,
            right: None,
            data,
        };
    }

    pub const fn get_data(&self) -> &T {
        return &self.data;
    }

    pub fn get_data_mut(&mut self) -> &mut T {
        return &mut self.data;
    }
}


impl<T: PartialOrd> Node<T> {
    pub fn insert(&mut self, data: T) {
        if self.data == data { return; }

        let target;
        if data < self.data { target = &mut self.left; }
        else { target = &mut self.right; }

        match target {
            &mut Some(ptr) => unsafe { (*ptr.as_ptr()).insert(data); },

            &mut None => unsafe {
                let node = Box::new(Node::new(data));
                *target = Some(NonNull::new_unchecked(Box::into_raw(node)));
            },
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


#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn create_node() {
        let node: Node<i32> = Node::new(0);
        assert_eq!(node.data, 0);

        let node: Node<f32> = Node::new(0.0);
        assert_eq!(node.data, 0.0);

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