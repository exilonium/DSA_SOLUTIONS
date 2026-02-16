impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack: Vec<char> = Vec::new();
        let mut k = k as usize;

        for ch in num.chars() {
            while k > 0 && !stack.is_empty() && *stack.last().unwrap() > ch {
                stack.pop();
                k -= 1;
            }
            stack.push(ch);
        }

        while k > 0 {
            stack.pop();
            k -= 1;
        }
        //removing the leading zero and building result
        let res: String = stack.into_iter().skip_while(|&c| c == '0').collect();
        if res.is_empty() { "0".to_string() } else { res }
    }
}
