// my approach 0ms
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        if n == 0 {
            return 1;
        }
        let mut num = n;
        let mut res = 0;
        let mut count = 0;
        while num > 0 {
            let bit = num & 1;
            match bit {
                0 => res += 2i32.pow(count),
                _ => {}
            }
            num >>= 1;
            count += 1;
        }
        res
    }
}
