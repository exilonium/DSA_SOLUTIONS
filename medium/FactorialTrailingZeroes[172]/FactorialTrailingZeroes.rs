impl Solution {
    pub fn trailing_zeroes(n: i32) -> i32 {
        let mut count = 0;
        let mut divisor = 5;
        while n >= divisor {
            count += n / divisor;
            divisor *= 5;
        }
        count
    }
}
