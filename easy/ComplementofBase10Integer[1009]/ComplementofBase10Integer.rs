impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let bitlength = 32 - n.leading_zeros();
        let mask = (1 << bitlength) - 1; // 10000 - 1  -> 01111

        n ^ mask
    }
}
