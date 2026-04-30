impl Solution {
    pub fn integer_break(n: i32) -> i32 {
        if n < 4 {
            n - 1
        } else {
            3i32.pow((n as u32 - 2) / 3) * ((n - 2) % 3 + 2)
        }
    }
}
