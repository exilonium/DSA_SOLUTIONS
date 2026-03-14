# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def isSubtree(self, root: Optional[TreeNode], subRoot: Optional[TreeNode]) -> bool:
        possible_roots = []
        search_node = subRoot.val

        def dfs(root):
            if not root:
                # print("fahh")
                return
            if root.val == search_node:
                possible_roots.append(root)
            dfs(root.left)
            # print(val)
            dfs(root.right)

        def weresame(root, subRoot):
            if not root and not subRoot:
                return True
            if not root or not subRoot:
                return False

            if root.val != subRoot.val:
                return False

            return weresame(root.left, subRoot.left) and weresame(
                root.right, subRoot.right
            )

        dfs(root)
        for i in possible_roots:
            if weresame(i, subRoot):
                return True
        return False
