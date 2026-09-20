impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut res = 0;

        for (i, c) in s.chars().enumerate() {
            let reverse_pos = 26 - (c as i32 - 'a' as i32);
            res += (i as i32 + 1) * reverse_pos;
        }

        res
    }
}
