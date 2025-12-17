impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        let mut x = n;
        if n <= 0 {
            return false;
        }
        while x % 3 == 0 {
            x = x / 3;
        }
        return x == 1;
    }
}
