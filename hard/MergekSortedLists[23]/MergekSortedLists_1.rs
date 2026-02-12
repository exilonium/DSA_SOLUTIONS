// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
//
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }

// this was 2ms of runtime cause we're pushing every element to the heap first
// also we're not utilising the fact that individual list is sorted.
use std::collections::BinaryHeap;
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pq = BinaryHeap::new();
        for i in lists {
            let mut list = i;
            while let Some(node) = list {
                pq.push(node.val);
                list = node.next;
            }
        }
        let mut res: Option<Box<ListNode>> = None;
        while let Some(val) = pq.pop() {
            let mut node = Box::new(ListNode::new(val));
            node.next = res;
            res = Some(node);
        }
        res
    }
}
