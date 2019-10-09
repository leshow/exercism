// 108. Convert Sorted Array to Binary Search Tree
// Easy
// Given an array where elements are sorted in ascending order, convert it to a
// height balanced BST. For this problem, a height-balanced binary tree is
// defined as a binary tree in which the depth of the two subtrees of every node
// never differ by more than 1. Example:
// Given the sorted array: [-10,-3,0,5,9],
// One possible answer is: [0,-3,9,-10,null,5], which represents the following
// height balanced BST:       0
//      / \
//    -3   9
//    /   /
//  -10  5
//  https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

use std::{cell::RefCell, rc::Rc};
pub fn sorted_array_to_bst<T: Copy>(nums: Vec<T>) -> Option<Rc<RefCell<TreeNode<T>>>> {
    if nums.is_empty() {
        None
    } else {
        make_bst(&nums[..])
    }
}
fn make_bst<T: Copy>(nums: &[T]) -> Option<Rc<RefCell<TreeNode<T>>>> {
    let mid = nums.len() >> 1;
    let mut node = TreeNode::new(nums[mid]);
    node.left = if mid > 0 {
        make_bst(&nums[..mid])
    } else {
        None
    };
    node.right = if mid + 1 < nums.len() {
        make_bst(&nums[(mid + 1)..])
    } else {
        None
    };
    Some(Rc::new(RefCell::new(node)))
}

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode<T> {
    pub val: T,
    pub left: Option<Rc<RefCell<TreeNode<T>>>>,
    pub right: Option<Rc<RefCell<TreeNode<T>>>>,
}
impl<T> TreeNode<T> {
    #[inline]
    pub fn new(val: T) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}
