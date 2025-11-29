// optimal
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stk= Vec::new();
        for i in s.chars(){
            match i {
                '(' => stk.push(')'),
                '{' => stk.push('}'),
                '[' => stk.push(']'),
                ')' | '}' | ']' if Some(i) != stk.pop() => return false,
                _ => (), // default case we need to provide rust
            }
        }
        return stk.is_empty();
    }
}
