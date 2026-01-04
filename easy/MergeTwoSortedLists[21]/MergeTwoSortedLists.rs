impl Solution {
    pub fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while l1.is_some() && l2.is_some() {
            let take_l1 = l1.as_ref().unwrap().val < l2.as_ref().unwrap().val;

            let mut node = if take_l1 {
                let mut n = l1.take().unwrap();
                l1 = n.next.take();
                n
            } else {
                let mut n = l2.take().unwrap();
                l2 = n.next.take();
                n
            };
            node.next = None;
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }
        tail.next = l1.or(l2);
        dummy.next
    }
}
