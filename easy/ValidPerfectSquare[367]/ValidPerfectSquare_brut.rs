// brut TLE
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        if num == 1 {
            return true;
        }
        for i in 0..=num / 2 {
            if i * i == num {
                return true;
            }
        }
        false
    }
}
