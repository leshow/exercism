// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct BSTIterator {
    unvisited: Vec<Rc<RefCell<TreeNode>>>,
}

use std::{cell::RefCell, rc::Rc};
/**
 ** `&self` means the method takes an immutable reference.
 ** If you need a mutable reference, change it to `&mut self` instead.
 **/
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut iter = BSTIterator { unvisited: vec![] };
        iter.push(root);
        iter
    }

    fn next(&mut self) -> i32 {
        let node = self.unvisited.pop().unwrap();
        let ret = node.borrow().val;
        self.push(node.borrow().right.clone());
        ret
    }

    fn has_next(&self) -> bool {
        !self.unvisited.is_empty()
    }

    fn push(&mut self, mut tree: Option<Rc<RefCell<TreeNode>>>) {
        while let Some(n) = tree.take() {
            self.unvisited.push(n.clone());
            tree = n.borrow().left.clone();
        }
    }
}
