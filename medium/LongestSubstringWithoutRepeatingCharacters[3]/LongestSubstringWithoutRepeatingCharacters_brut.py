class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        i = 0
        j = 0
        myset = set()
        count = 0

        while j < len(s):
            if s[j] not in myset:
                myset.add(s[j])
                count = max(count, j - i + 1)
                j += 1
            else:
                myset.remove(s[i])
                i += 1

        return count
