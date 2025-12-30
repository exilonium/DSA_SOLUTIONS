impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        // overflow case
        if dividend == i32::MIN && divisor == -1 {
            return i32::MAX;
        }

        let is_neg = (dividend < 0) ^ (divisor < 0);

        let mut p = if dividend > 0 { -dividend } else { dividend };
        let q = if divisor > 0 { -divisor } else { divisor };

        let mut res = 0;

        while p <= q {
            let mut temp = q;
            let mut count = -1;

            // double temp safely to avoid overflow
            while temp >= (i32::MIN >> 1) && p <= (temp << 1) {
                temp <<= 1;
                count <<= 1;
            }

            p -= temp;
            res += count;
        }

        if is_neg { res } else { -res }
    }
}
