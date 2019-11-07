use std::{cell::RefCell, rc::Rc};
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
/// 701. Insert into a Binary Search Tree
/// Medium
///
/// Given the root node of a binary search tree (BST) and a value to be inserted
/// into the tree, insert the value into the BST. Return the root node of the
/// BST after the insertion. It is guaranteed that the new value does not exist
/// in the original BST.
///
/// Note that there may exist multiple valid ways for the insertion, as long as
/// the tree remains a BST after insertion. You can return any of them.
///
/// For example,
///
/// Given the tree:
///         4
///        / \
///       2   7
///      / \
///     1   3
/// And the value to insert: 5
pub fn insert_into_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    val: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    use std::cmp::Ordering;
    fn insert(root: &Option<Rc<RefCell<TreeNode>>>, val: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let mut nref = node.borrow_mut();
                match nref.val.cmp(&val) {
                    Ordering::Greater => nref.left = insert(&nref.left, val),
                    Ordering::Less => nref.right = insert(&nref.right, val),
                    Ordering::Equal => {}
                }
                Some(node.clone())
            }
            None => Some(Rc::new(RefCell::new(TreeNode::new(val)))),
        }
    }
    insert(&root, val);
    root
}
