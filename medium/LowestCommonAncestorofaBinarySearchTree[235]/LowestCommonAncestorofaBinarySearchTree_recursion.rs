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
    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        let p_val = p.unwrap().borrow().val;
        let q_val = q.unwrap().borrow().val;
        fn dfs(
            root: &Option<Rc<RefCell<TreeNode>>>,
            p: i32,
            q: i32,
        ) -> Option<Rc<RefCell<TreeNode>>> {
            if let Some(node) = root {
                let n = node.borrow();
                if n.val > p && n.val > q {
                    return dfs(&n.left, p, q);
                }
                if n.val < p && n.val < q {
                    return dfs(&n.right, p, q);
                }
                return Some(node.clone());
            } else {
                None
            }
        }
        dfs(&root, p_val, q_val)
    }
}
