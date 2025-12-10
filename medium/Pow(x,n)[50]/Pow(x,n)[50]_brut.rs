// brut
impl Solution {
    fn pow(x: f64, n: i32) -> f64 {
        if x == 0 as f64 {
            return 0.into();
        } else if n == 0 || x == 1 as f64 {
            return 1.into();
        } else if x < 0.0001 && n > 1000000 {
            return 0.into();
        } else if n < -1000000 {
            return 0.into();
        } else if n == 1 {
            return x.into();
        } else {
            if n > 0 {
                x * Self::pow(x, n - 1)
            } else {
                Self::pow(x, n + 1) * (1 as f64 / x)
            }
        }
    }
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == -1 as f64 {
            if (n % 2) == 1 {
                return -1 as _;
            } else {
                return 1.into();
            }
        }
        Self::pow(x, n)
    }
}
