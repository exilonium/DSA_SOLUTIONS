// 0 ms

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

impl Solution {
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut myhead = head.as_ref();
        let mut count = 0;
        while let Some(node) = myhead {
            count += 1;
            myhead = node.next.as_ref();
        }
        let mut current = head;
        for i in 0..count / 2 {
            if let Some(node) = current {
                current = node.next;
            }
        }
        current
    }
}
