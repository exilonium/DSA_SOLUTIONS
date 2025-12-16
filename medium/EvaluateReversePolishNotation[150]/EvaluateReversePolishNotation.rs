// optimal
impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stk = vec![];
        for i in tokens {
            match i.as_str() {
                "+" => {
                    let b = stk.pop().unwrap();
                    let a = stk.pop().unwrap();
                    stk.push(a + b);
                }
                "-" => {
                    let b = stk.pop().unwrap();
                    let a = stk.pop().unwrap();
                    stk.push(a - b);
                }
                "*" => {
                    let b = stk.pop().unwrap();
                    let a = stk.pop().unwrap();
                    stk.push(a * b);
                }
                "/" => {
                    let b = stk.pop().unwrap();
                    let a = stk.pop().unwrap();
                    stk.push(a / b);
                }
                _ => {
                    let a = i.parse::<i32>().unwrap();
                    stk.push(a);
                }
            }
        }
        stk.pop().unwrap_or(0)
    }
}
