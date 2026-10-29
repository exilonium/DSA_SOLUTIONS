impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut first = [-1; 26];
        let mut ans = -1;

        for (i, c) in s.bytes().enumerate() {
            let idx = (c - b'a') as usize;

            if first[idx] == -1 {
                first[idx] = i as i32;
            } else {
                ans = ans.max(i as i32 - first[idx] - 1);
            }
        }

        ans
    }
}
