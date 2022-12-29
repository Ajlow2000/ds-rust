#![allow(dead_code)]
#![allow(unused_variables)]

type Child<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    data: T,
    left: Child<T>,
    right: Child<T>,
}

#[derive(Debug)]
struct BST<T> {
    root: Child<T>,
}

impl<T> BST<T> {
    fn new(r: Child<T>) -> BST<T> {
        BST { root: r }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn insert(&self, data: T) {
        todo!()
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
 I  }

    #[test]
    fn constructor_none() {
        let tree: BST<i32> = BST::new(None);
        assert_eq!(tree.is_empty(), true);
    }

    #[test]
    fn constructor_some() {
        let c = Some(Box::new(Node {
            data: 3,
            left: None,
            right: None,
        }));
        let tree: BST<i32> = BST::new(c);
        assert_eq!(tree.is_empty(), false);
    }

    #[test]
    fn insert_into_empty_tree() {
        let tree: BST<i32> = BST::new(None);
        tree.insert(5);
        assert!(!tree.is_empty());
    }

    // #[test]
    // fn insert_into_non_empty_tree() {
    //     let c = Some(Box::new(Node{data: 3, left: None, right: None}));
    //     let tree: BST<i32> = BST::new(c);
    //     tree.insert(5);
    //     // assert_eq!(tree.root.right.expect("Root's right child is not the expected value"), 5);
    //     assert_eq!(*(tree.root.unwrap().right.expect("Unexpected right child of root")), 5);
    // }
}
