impl Solution {
    pub fn calculate(s: String) -> i32 {
        let mut stk = vec![];
        let mut res = 0;
        let mut num = 0;
        let mut sign = 1; // 1 is plus -1 is minus
        for c in s.chars() {
            match c {
                '0'..='9' => {
                    num = num * 10 + (c as i32 - '0' as i32);
                }
                '+' => {
                    res += sign * num;
                    num = 0;
                    sign = 1;
                }
                '-' => {
                    res += sign * num;
                    num = 0;
                    sign = -1;
                }
                '(' => {
                    stk.push(res);
                    stk.push(sign);
                    res = 0;
                    sign = 1;
                }
                ')' => {
                    res += sign * num;
                    num = 0;

                    let prev_sign = stk.pop().unwrap();
                    let prev_res = stk.pop().unwrap();

                    res = prev_res + prev_sign * res;
                }

                ' ' => {} // ignore spaces

                _ => {}
            }
        }
        res = res + sign * num; // processing the last number in case we are not having the bracket
        res
    }
}
