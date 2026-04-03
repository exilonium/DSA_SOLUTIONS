class Solution:
    def reverseWords(self, s: str) -> str:
        res = []
        for i in s.split(" "):
            if i != "":
                res.insert(0, i)
        return " ".join(res).strip()
