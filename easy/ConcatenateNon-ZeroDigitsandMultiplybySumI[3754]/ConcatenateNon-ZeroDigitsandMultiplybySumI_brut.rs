impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let temp: String = n.to_string().chars().filter(|&c| c != '0').collect();
        if temp.is_empty() {
            return 0;
        }
        let num = temp.parse::<i64>().unwrap();
        let sum: i64 = temp.chars().map(|c| c.to_digit(10).unwrap() as i64).sum();

        num * sum
    }
}
