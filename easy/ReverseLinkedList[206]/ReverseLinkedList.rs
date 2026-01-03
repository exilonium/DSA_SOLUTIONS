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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut prev = None; // head of the reversed list (initially empty)
        let mut curr = head; // head of the remaining list to process

        while let Some(mut node) = curr {
            // detach the next node and keep the rest of the list
            curr = node.next.take();

            // reverse the pointer so this node points to the reversed list
            node.next = prev;

            // move the head of the reversed list forward
            prev = Some(node);
        }

        // prev is now the new head of the reversed list
        prev
    }
}
