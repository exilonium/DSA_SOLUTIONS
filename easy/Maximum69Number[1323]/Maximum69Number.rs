impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut n: Vec<i32> = num
            .to_string()
            .chars()
            .map(|x| x.to_digit(10).unwrap_or(9) as i32)
            .collect();
        for i in 0..n.len() {
            if n[i] == 6 {
                n[i] = 9;
                break;
            }
        }
        let mut res = 0i32;
        for digit in n {
            res = res * 10 + digit;
        }
        res
    }
}
