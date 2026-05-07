// not my solution
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
use std::collections::VecDeque;
use std::rc::Rc;

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut queue = VecDeque::from([(root, 0)]);
        let mut levels: Vec<Vec<i32>> = Vec::new();

        // Perform a breadth-first search
        while let Some((node, level)) = queue.pop_front() {
            let Some(mut node) = node.as_ref().map(|n| n.borrow_mut()) else {
                continue;
            };

            // Check if the vector for the level has been added
            // Insert it with the value if it hasn't, push the value otherwise
            if levels.len() == level + 1 {
                levels[level].push(node.val);
            } else {
                levels.push(vec![node.val]);
            }

            queue.push_back((node.left.take(), level + 1));
            queue.push_back((node.right.take(), level + 1));
        }
        levels
    }
}
