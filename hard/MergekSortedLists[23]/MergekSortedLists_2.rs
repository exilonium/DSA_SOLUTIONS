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

// this is slightly slower than the other approach but it taught me a lot of stuff
// 3ms
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd<ListNode> for ListNode {
    fn partial_cmp(&self, other: &ListNode) -> Option<Ordering> {
        other.val.partial_cmp(&self.val) // for min heap
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.val.cmp(&self.val) // for min heap
    }
}
impl Solution {
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut pq = BinaryHeap::new();
        for list in lists {
            match list {
                Some(node) => pq.push(node),
                None => {}
            }
        }

        let mut res = Box::new(ListNode::new(0));
        let mut tail = &mut res;
        while let Some(node) = pq.pop() {
            let mut temp = Box::new(ListNode::new(node.val));
            tail.next = Some(temp);
            tail = tail.next.as_mut().unwrap();
            if let Some(next) = node.next {
                pq.push(next);
            }
        }
        res.next
    }
}
