class Solution:
    def minDeletionSize(self, strs: List[str]) -> int:
        count = 0
        for i in range(len(strs[0])):
            prev = ""
            for idx in range(len(strs)):
                curr = strs[idx][i]
                if curr < prev:
                    count += 1
                    break
                prev = curr
        return count
