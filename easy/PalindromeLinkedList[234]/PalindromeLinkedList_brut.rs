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
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let mut values: Vec<i32> = Vec::new();
        let mut current = head.as_ref();
        while let Some(node) = current {
            values.push(node.val);
            current = node.next.as_ref();
        }
        let mut i = 0;
        let mut j = values.len() - 1;
        while i < j {
            if values[i] != values[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        true
    }
}
