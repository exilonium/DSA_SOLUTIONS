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
    pub fn odd_even_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut head = head;
        let (mut res, mut second) = (None, None);
        let (mut tail1, mut tail2) = (&mut res, &mut second);
        let mut flag = false;
        while let Some(mut node) = head {
            head = node.next;
            node.next = None;
            flag = !flag;
            if flag {
                tail1 = &mut tail1.insert(node).next;
            } else {
                tail2 = &mut tail2.insert(node).next;
            }
        }
        if let Some(node) = second {
            tail1 = &mut tail1.insert(node).next;
        }
        res
    }
}
