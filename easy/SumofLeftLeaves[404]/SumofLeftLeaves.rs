// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    pub fn sum_of_left_leaves(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn solve(root: &Option<Rc<RefCell<TreeNode>>>, l: bool) -> i32 {
            if let Some(r) = root {
                let r = r.borrow();
                if l && r.left.is_none() && r.right.is_none() {
                    r.val
                } else {
                    solve(&r.left, true) + solve(&r.right, false)
                }
            } else {
                0
            }
        }
        solve(&root, false)
    }
}
