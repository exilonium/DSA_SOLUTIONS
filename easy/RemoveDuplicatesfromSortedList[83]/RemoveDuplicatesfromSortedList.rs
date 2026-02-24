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
    pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let mut curr_opt = head.as_mut();

        while let Some(curr) = curr_opt {
            let mut next_opt = curr.next.take();

            while let Some(next) = next_opt.as_mut() {
                if next.val == curr.val {
                    next_opt = next.next.take();
                } else {
                    curr.next = next_opt;
                    break;
                }
            }
            curr_opt = curr.next.as_mut();
        }
        head
    }
}
