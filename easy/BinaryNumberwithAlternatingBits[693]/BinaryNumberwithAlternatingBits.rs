impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n;
        let mut prev = n & 1;
        n >>= 1;
        while n > 0 {
            let curr = n & 1;
            if curr == prev {
                return false;
            }
            prev = curr;
            n >>= 1;
        }
        true
    }
}
