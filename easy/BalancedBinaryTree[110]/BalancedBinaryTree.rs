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
    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn dfs(node: Option<Rc<RefCell<TreeNode>>>) -> i32 {
            match node {
                None => 0,
                Some(nd) => {
                    let n = nd.borrow();
                    let left = dfs(n.left.clone());
                    if left == -1 {
                        return -1;
                    }
                    let right = dfs(n.right.clone());
                    if right == -1 {
                        return -1;
                    }
                    if (left - right).abs() > 1 {
                        return -1;
                    }
                    1 + left.max(right)
                }
            }
        }
        dfs(root) != -1
    }
}
