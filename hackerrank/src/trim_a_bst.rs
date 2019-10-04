// 669. Trim a Binary Search Tree
// Easy
// Given a binary search tree and the lowest and highest boundaries as L and R,
// trim the tree so that all its elements lies in [L, R] (R >= L). You might
// need to change the root of the tree, so the result should return the new root
// of the trimmed binary search tree.

// Example 1:

// Input:
//     1
//    / \
//   0   2

//   L = 1
//   R = 2

// Output:
//     1
//       \
//        2

// Example 2:

// Input:
//     3
//    / \
//   0   4
//    \
//     2
//    /
//   1

//   L = 1
//   R = 3

// Output:
//       3
//      /
//    2
//   /
//  1

pub fn trim_bst(
    root: Option<Rc<RefCell<TreeNode>>>,
    l: i32,
    r: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    fn trim(root: &Option<Rc<RefCell<TreeNode>>>, l: i32, r: i32) -> Option<Rc<RefCell<TreeNode>>> {
        match root {
            Some(node) => {
                let mut nref = node.borrow_mut();
                if nref.val > r {
                    return trim(&nref.left, l, r);
                }
                if nref.val < l {
                    return trim(&nref.right, l, r);
                }
                nref.left = trim(&nref.left, l, r);
                nref.right = trim(&nref.right, l, r);
                Some(node.clone())
            }
            None => None,
        }
    }
    trim(&root, l, r);
    root
}
