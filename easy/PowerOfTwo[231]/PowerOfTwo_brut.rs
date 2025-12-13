// this code works but didnt pass due to the maximum recursion depth
impl Solution {
    fn divit(n: i32) -> i32 {
        if n <= 2 { n } else { Self::divit(n / 2) }
    }

    pub fn is_power_of_two(n: i32) -> bool {
        if n <= 0 {
            return false;
        }
        if n == 1 {
            return true;
        }
        Self::divit(n) == 2
    }
}
