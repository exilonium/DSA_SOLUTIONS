"""
# Definition for a Node.
class Node:
    def __init__(self, x: int, next: 'Node' = None, random: 'Node' = None):
        self.val = int(x)
        self.next = next
        self.random = random
"""


class Solution:
    def copyRandomList(self, head: "Optional[Node]") -> "Optional[Node]":
        hash = {None: None}  # storing hash value in hashmap/dictionary
        curr = head
        while curr:
            hash[curr] = Node(curr.val)  # mapping dictionary current to current value
            curr = curr.next
        curr = head
        while curr:
            tmp = hash[curr]
            tmp.next = hash[curr.next]
            tmp.random = hash[curr.random]
            curr = curr.next
        return hash[head]
