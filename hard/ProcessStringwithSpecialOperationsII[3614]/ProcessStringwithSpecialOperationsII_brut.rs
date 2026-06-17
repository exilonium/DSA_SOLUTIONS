// TLE but a good poc of how to not write code
impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let mut res = String::new();
        for ch in s.chars() {
            match ch {
                '*' => {
                    if res.len() != 0 {
                        res.pop();
                    }
                }
                '#' => {
                    res = res.repeat(2);
                }
                '%' => {
                    res = res.chars().rev().collect();
                }
                x => {
                    res.push(x);
                }
            }
        }

        res.chars().nth(k as usize).unwrap_or('.')
    }
}
