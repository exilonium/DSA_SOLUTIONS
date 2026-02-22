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
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut root = root.clone();
        let mut stack = vec![];
        let mut res = vec![];
        while root.is_some() || !stack.is_empty() {
            while let Some(node) = root {
                stack.push(node.clone());
                root = node.borrow().left.clone();
            }
            if let Some(node) = stack.pop() {
                let n = node.borrow();
                res.push(n.val);
                root = n.right.clone();
            }
        }
        res
    }
}
