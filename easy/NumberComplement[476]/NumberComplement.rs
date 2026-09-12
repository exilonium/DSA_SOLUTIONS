impl Solution {
    pub fn find_complement(mut num: i32) -> i32 {
        let mut res = 0;
        let mut bit = 1;

        while num > 0 {
            if num & 1 == 0 {
                res |= bit;
            }

            num >>= 1;
            bit <<= 1;
        }

        res
    }
}
