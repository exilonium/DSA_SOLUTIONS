# i was having some hard time with the rust so i made a quick prototype in python
class Solution:
    def evalRPN(self, tokens: List[str]) -> int:
        stk = []
        for i in tokens:
            match i:
                case "+":
                    b = stk.pop()
                    a = stk.pop()
                    stk.append(a + b)
                case "-":
                    b = stk.pop()
                    a = stk.pop()
                    stk.append(a - b)
                case "/":
                    b = stk.pop()
                    a = stk.pop()
                    stk.append(int(a / b))
                case "*":
                    b = stk.pop()
                    a = stk.pop()
                    stk.append(a * b)
                case _:
                    stk.append(int(i))

        return int(stk[0])
