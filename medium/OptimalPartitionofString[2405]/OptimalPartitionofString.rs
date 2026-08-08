impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut mask = 0u32;
        let mut ans = 1;

        for b in s.bytes() {
            let bit = 1 << (b - b'a');

            if mask & bit != 0 {
                ans += 1;
                mask = 0;
            }

            mask |= bit;
        }

        ans
    }
}
