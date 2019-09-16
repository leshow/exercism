// 965. Univalued Binary Tree
// Easy

// A binary tree is univalued if every node in the tree has the same value.

// Return true if and only if the given tree is univalued.

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
use std::{cell::RefCell, rc::Rc};
pub fn is_unival_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn same_val(root: &Option<Rc<RefCell<TreeNode>>>) -> bool {
        match root {
            Some(n) => {
                let left = if let Some(ref l) = n.borrow().left {
                    n.borrow().val == l.borrow().val && same_val(&n.borrow().left)
                } else {
                    true
                };
                let right = if let Some(ref r) = n.borrow().right {
                    n.borrow().val == r.borrow().val && same_val(&n.borrow().right)
                } else {
                    true
                };
                left && right
            }
            None => true,
        }
    }
    same_val(&root)
}

pub fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    if root.is_none() {
        return true;
    }
    let root = root.unwrap();
    let mut stack = vec![&root];
    while !stack.is_empty() {
        let node = stack.pop().unwrap();
        let node = node.borrow();

        if let Some(ref l) = node.left {
            if node.val != l.borrow().val {
                return false;
            }
            stack.push(l);
        }
        if let Some(ref r) = node.right {
            if node.val != r.borrow().val {
                return false;
            }
            stack.push(r);
        }
    }
    true
}
