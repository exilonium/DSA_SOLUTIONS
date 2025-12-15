impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut n = n;
        let mut count = 0;
        while n != 0 {
            count += n & 1;
            n = n >> 1;
        }
        count
    }
}
