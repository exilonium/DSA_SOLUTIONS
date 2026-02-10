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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // Create dummy node to simplify edge cases
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));

        //get length
        let mut len = 0;
        let mut p = dummy.as_ref();
        while let Some(node) = p {
            p = node.next.as_ref();
            len += 1;
        }

        // Find node BEFORE the one we remove
        let mut cur = dummy.as_mut();
        for _ in 0..(len - n as usize - 1) {
            cur = cur.unwrap().next.as_mut();
        }

        // Remove target node
        if let Some(node) = cur {
            let next = node.next.as_mut().and_then(|x| x.next.take());
            node.next = next;
        }

        dummy.unwrap().next
    }
}
