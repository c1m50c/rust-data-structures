pub(crate) mod node;

use node::Node;

use std::option::Option;
use std::ptr::NonNull;


/// Struct for creating a `BinaryTree`.
/// ## Fields:
/// ```rust
/// root: Option<NonNull<Node<T>>> // Root Node of the Tree.
/// ```
#[derive(Debug)]
pub struct BinaryTree<T> {
    root: Option<NonNull<Node<T>>>,
}


impl<T> BinaryTree<T> {
    pub const fn new() -> Self {
        return Self {
            root: None,
        }
    }
}


impl<T: PartialOrd> BinaryTree<T> {
    pub fn insert(&mut self, data: T) {
        match self.root {
            Some(mut ptr) => unsafe { ptr.as_mut().insert(data); },

            None => unsafe {
                let node = Box::new(Node::new(data));
                self.root = Some(NonNull::new_unchecked(Box::into_raw(node)));
            },
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

    #[test]
    #[ignore]
    fn insert() {
        // TODO: Complete Test
        let mut tree = BinaryTree::new();
        tree.insert(10);
        tree.insert(15);
        tree.insert(5);
        tree.insert(20);
    }
}