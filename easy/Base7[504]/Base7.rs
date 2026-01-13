impl Solution {
    pub fn convert_to_base7(num: i32) -> String {
        let mut num = num;
        let mut res = 0;
        let mut base = 1;
        while num != 0 {
            res = res + base * (num % 7);
            base *= 10;
            num /= 7;
        }
        res.to_string()
    }
}
