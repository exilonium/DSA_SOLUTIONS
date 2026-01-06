# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def reverseBetween(
        self, head: Optional[ListNode], left: int, right: int
    ) -> Optional[ListNode]:
        if not head or left == right:
            return head

        dummy = ListNode(0)
        dummy.next = head
        prev = dummy

        # move prev to node before left
        for _ in range(left - 1):
            prev = prev.next

        # reverse sublist
        curr = prev.next
        prev_sub = None

        for _ in range(right - left + 1):
            next_node = curr.next
            curr.next = prev_sub
            prev_sub = curr
            curr = next_node

        # reconnect
        prev.next.next = curr  # tail of reversed → rest
        prev.next = prev_sub  # before left → new head

        return dummy.next
