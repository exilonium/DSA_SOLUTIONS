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
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res = vec![];
        let mut bfs = vec![];

        if let Some(node) = root {
            bfs.push(node);
        }

        while bfs.len() != 0 {
            res.push(bfs[bfs.len() - 1].borrow().val);

            let mut row = vec![];

            for node in bfs.iter() {
                if let Some(ref left) = node.borrow().left {
                    row.push(Rc::clone(left));
                }

                if let Some(ref right) = node.borrow().right {
                    row.push(Rc::clone(right));
                }
            }

            bfs = row;
        }
        res
    }
}
