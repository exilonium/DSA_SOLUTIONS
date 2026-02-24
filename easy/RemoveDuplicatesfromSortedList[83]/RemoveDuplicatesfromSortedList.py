# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, val=0, next=None):
#         self.val = val
#         self.next = next
class Solution:
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        tail = head
        while tail and tail.next:
            inc = True
            if tail.val == tail.next.val:
                tail.next = tail.next.next
                inc = False
            if inc:
                tail = tail.next

        return head
