# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def inorderTraversal(self, root: Optional[TreeNode]) -> List[int]:
        res = []

        def traverse(root):
            if not root:
                return []
            left = root.left
            right = root.right
            traverse(left)
            res.append(root.val)
            traverse(right)

        traverse(root)
        return res
