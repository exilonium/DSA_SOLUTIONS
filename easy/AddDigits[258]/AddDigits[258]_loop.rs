impl Solution {
    pub fn add_digits(n: i32) -> i32 {
        let mut i = n;
        let mut sum = 0;
        loop {
            while i > 0 {
                sum += i % 10;
                i = i / 10;
            }
            if sum < 10 {
                break;
            }
            i = sum;
            sum = 0;
        }
        sum
    }
}
