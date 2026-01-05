# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        done = set()
        current = head
        while current is not None:
            addr = current.next
            if addr in done:
                return True
            done.add(addr)
            current = addr
        return False
