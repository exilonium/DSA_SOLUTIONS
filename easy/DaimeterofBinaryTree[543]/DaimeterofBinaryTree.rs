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
// idfking understand how this code works :)
impl Solution {
    pub fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32) {
            match root {
                None => (0, 0),
                Some(node) => {
                    let (ld, ldia) = dfs(node.borrow().left.clone());
                    let (rd, rdia) = dfs(node.borrow().right.clone());
                    (
                        i32::max(ld, rd) + 1,
                        i32::max(ldia, i32::max(rdia, ld + rd)),
                    )
                }
            }
        }
        dfs(root).1 // .1 means its accessing the 1st element of the tuple
    }
}
