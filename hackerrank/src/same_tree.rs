// 100. Same Tree
// Easy
// Given two binary trees, write a function to check if they are the same or not.
// Two binary trees are considered the same if they are structurally identical and the nodes have the same value.
// Example 1:
// Input:     1         1
//           / \       / \
//          2   3     2   3
//         [1,2,3],   [1,2,3]
// Output: true
// Example 2:
// Input:     1         1
//           /           \
//          2             2
//         [1,2],     [1,null,2]
// Output: false
// Example 3:
// Input:     1         1
//           / \       / \
//          2   1     1   2
//         [1,2,1],   [1,1,2]
// Output: false

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
use std::cell::RefCell;
use std::rc::Rc;
pub fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    eq(&p, &q)
}
fn eq(a: &Option<Rc<RefCell<TreeNode>>>, b: &Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (a, b) {
        (Some(ax), Some(bx)) if ax.borrow().val == bx.borrow().val => {
            eq(&ax.borrow().left, &bx.borrow().left) && eq(&ax.borrow().right, &bx.borrow().right)
        }
        (None, None) => true,
        _ => false,
    }
}
