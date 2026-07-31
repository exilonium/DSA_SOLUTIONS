impl Solution {
    pub fn maximum_odd_binary_number(s: String) -> String {
        let mut one = 0;
        let mut zero = 0;
        for i in s.chars() {
            match i {
                '1' => one += 1,
                '0' => zero += 1,
                _ => (),
            }
        }
        let mut res = String::with_capacity(s.len());
        for _ in 0..one - 1 {
            res.push('1');
        }
        for _ in 0..zero {
            res.push('0');
        }
        res.push('1');
        res
    }
}
