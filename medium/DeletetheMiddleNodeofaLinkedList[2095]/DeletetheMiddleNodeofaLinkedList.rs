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
    pub fn delete_middle(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut fast = &(head.clone());
        let mut slow = &mut head;

        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &mut (slow.as_mut()?.next);
            fast = &(fast.as_ref()?.next.as_ref()?.next);
        }

        *slow = (*slow).as_mut()?.next.take();

        head
    }
}
