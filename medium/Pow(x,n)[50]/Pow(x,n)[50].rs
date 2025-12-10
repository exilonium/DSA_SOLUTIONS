//optimal
impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        let mut x = x;
        let mut exp = n as i64;
        if exp < 0 {
            x = 1.0 / x;
            exp = -exp;
        }
        let mut result = 1.0;
        while exp > 0 {
            if exp & 1 == 1 {
                // checks if n is odd (last bit is one)
                result *= x;
            }
            x *= x;
            exp >>= 1; // this is basically dividing by two using bitwise operator
        }
        result
    }
}
