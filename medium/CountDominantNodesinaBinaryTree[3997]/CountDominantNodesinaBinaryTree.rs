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
    pub fn count_dominant_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            let Some(node) = node else {
                return (i32::MIN, 0);
            };

            let node = node.borrow();

            let (left_max, left_count) = dfs(node.left.clone());
            let (right_max, right_count) = dfs(node.right.clone());

            let subtree_max = node.val.max(left_max).max(right_max);

            let count = left_count + right_count + if node.val == subtree_max { 1 } else { 0 };

            (subtree_max, count)
        }

        dfs(root).1
    }
}
