pub(crate) mod node;

use node::Node;

use std::option::Option;
use std::ptr::NonNull;


/// Struct for creating a `BinaryTree`.
/// ## Fields:
/// ```rust
/// pub root: Option<NonNull<Node<T>>> // Root Node of the Tree.
/// ```
#[derive(Debug)]
pub struct BinaryTree<T> {
    pub root: Option<NonNull<Node<T>>>,
}


impl<T> BinaryTree<T> {
    pub const fn new() -> Self {
        return Self {
            root: None,
        }
    }
}


impl<T> Default for BinaryTree<T> {
    fn default() -> Self {
        return Self::new();
    }
}


#[cfg(test)]
mod tests {
    use super::BinaryTree;

    #[test]
    fn default() {
        let tree: BinaryTree<i32> = BinaryTree::default();
        assert_eq!(tree.root, None);
    }
}