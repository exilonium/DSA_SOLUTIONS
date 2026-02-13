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
    fn mirror(root: &mut Option<Rc<RefCell<TreeNode>>>) {
        if let Some(node) = root {
            let mut node_ref = node.borrow_mut();
            Self::mirror(&mut node_ref.left);
            Self::mirror(&mut node_ref.right);
            let left = node_ref.left.take();
            let right = node_ref.right.take();
            node_ref.left = right;
            node_ref.right = left;
        }
    }
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(node) = root {
            let clone = Rc::clone(&node);
            Self::mirror(&mut Some(clone));
            Some(node)
        } else {
            None
        }
    }
}
