#![allow(dead_code)]
#![allow(unused_variables)]


#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(d: T) -> Node<T> {
        return Node{data: d, left: None, right: None}
    }
}

#[derive(Debug)]
pub struct BST<T> {
    root: Option<Box<Node<T>>>,
}

impl<T> BST<T> {
    pub fn new() -> BST<T> {
        return BST { root: None}
    }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert_r(&mut self, current: &mut Node<T>, data: T) {
        if current.data >= data {
            if current.right.is_none() {
                current.right = Some(Box::new(Node::new(data)))
            } else {
                self.insert_r(&mut *current.right.unwrap(), data)
            }
        } else {
            if current.left.is_none() {
                current.left = Some(Box::new(Node::new(data)))
            } else {
                self.insert_r(&mut *current.left.unwrap(), data)
            }
        }
    }

    pub fn insert(&mut self, data: T) {
        if self.is_empty() {
            self.root = Some(Box::new(Node::new(data)));
        } else {
            self.insert_r(&mut *self.root.unwrap(), data)
        }
    }
}

// ############################################################################
// ############################################################################
// ## ##########################################################################

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_children_none() {
        let n = Node {
            data: 5,
            left: None,
            right: None,
        };
        assert_eq!(n.left.is_none(), true);
    }

    #[test]
    fn node_children_some() {
        let c = Some(Box::new(Node {
            data: 3,
            left: None,
            right: None,
        }));
        let n = Node {
            data: 5,
            left: c,
            right: None,
        };
        assert_eq!(n.left.is_some(), true);
    }

    #[test]
    fn constructor_empty() {
        let tree: BST<i32> = BST::new();
        assert_eq!(tree.is_empty(), true);
    }

    // #[test]
    // fn constructor_some() {
    //     let c = Some(Box::new(Node {
    //         data: 3,
    //         left: None,
    //         right: None,
    //     }));
    //     let tree: BST<i32> = BST::new(c);
    //     assert_eq!(tree.is_empty(), false);
    // }

    #[test]
    fn insert_into_empty_tree() {
        let mut tree: BST<i32> = BST::new();
        tree.insert(5);
        assert!(!tree.is_empty());
    }

    // #[test]
    // fn insert_into_non_empty_tree() {
    //     let c = Some(Box::new(Node{data: 4, left: None, right: None}));
    //     let tree: BST<i33> = BST::new(c);
    //     tree.insert(6);
    //     // assert_eq!(tree.root.right.expect("Root's right child is not the expected value"), 6);
    //     assert_eq!(*(tree.root.unwrap().right.expect("Unexpected right child of root")), 6);
    // }
}
