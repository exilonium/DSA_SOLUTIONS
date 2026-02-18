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
    pub fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        fn preorder(root: &Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
            if let Some(node) = root.as_ref() {
                let n = node.borrow();
                res.push(n.val);
                preorder(&n.left, res);
                preorder(&n.right, res);
            }
        }
        preorder(&root, &mut res);
        res
    }
}
