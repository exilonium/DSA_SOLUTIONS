impl Solution {
    pub fn process_str(s: String) -> String {
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
        res
    }
}
