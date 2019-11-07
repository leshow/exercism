// 654. Maximum Binary Tree
// Medium

// Given an integer array with no duplicates. A maximum tree building on this
// array is defined as follow:

//     The root is the maximum number in the array.
//     The left subtree is the maximum tree constructed from left part subarray
// divided by the maximum number.     The right subtree is the maximum tree
// constructed from right part subarray divided by the maximum number.

// Construct the maximum tree by the given array and output the root node of
// this tree.

// Example 1:

// Input: [3,2,1,6,0,5]
// Output: return the tree root node representing the following tree:

//       6
//     /   \
//    3     5
//     \    /
//      2  0
//        \
//         1

use std::{cell::RefCell, cmp, rc::Rc};

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

pub fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    go(&nums)
}

fn go(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }
    let max = nums
        .iter()
        .fold(i32::min_value(), |max, &v| cmp::max(max, v));
    let i = nums.iter().position(|&x| x == max).unwrap();
    let mut node = TreeNode::new(nums[i]);
    node.left = go(&nums[..i]);
    node.right = go(&nums[(i + 1)..]);
    Some(Rc::new(RefCell::new(node)))
}
