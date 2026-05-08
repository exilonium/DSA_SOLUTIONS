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
    pub fn rob(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let ret = Solution::dfs(&root);
        std::cmp::max(ret.0, ret.1)
    }
    fn dfs(node: &Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
        if let Some(n) = node {
            let l = Solution::dfs(&n.borrow().left);
            let r = Solution::dfs(&n.borrow().right);
            (
                n.borrow().val + l.1 + r.1,
                std::cmp::max(l.0, l.1) + std::cmp::max(r.0, r.1),
            )
        } else {
            (0, 0)
        }
    }
}
