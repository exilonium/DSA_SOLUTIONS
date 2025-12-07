// first try 0ms
impl Solution {
    fn dig_sum(mut number: i32) -> i32 {
        let mut res = 0;
        while number != 0 {
            let digit = number % 10;
            number /= 10;
            res += digit;
        }
        res
    }
    pub fn add_digits(num: i32) -> i32 {
        let mut res = num;
        while res > 9 {
            res = Self::dig_sum(res);
        }
        res
    }
}
