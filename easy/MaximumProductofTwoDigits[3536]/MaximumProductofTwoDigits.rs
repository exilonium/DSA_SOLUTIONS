impl Solution {
    pub fn max_product(n: i32) -> i32 {
        let mut n = n;
        let mut m: u64 = 0;

        while n > 0 {
            let d = (n % 10) as u32;
            m += 1u64 << (d << 2);
            n /= 10;
        }

        let first = (63 - m.leading_zeros()) >> 2;
        m -= 1u64 << (first << 2);
        let second = (63 - m.leading_zeros()) >> 2;

        (first * second) as i32
    }
}
