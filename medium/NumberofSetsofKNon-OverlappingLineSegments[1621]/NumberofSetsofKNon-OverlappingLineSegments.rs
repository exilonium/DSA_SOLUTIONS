impl Solution {
    const MOD: i64 = 1_000_000_007;

    fn power(mut a: i64, mut b: i64) -> i64 {
        let mut res = 1;

        while b > 0 {
            if b & 1 == 1 {
                res = res * a % Self::MOD;
            }

            a = a * a % Self::MOD;
            b >>= 1;
        }

        res
    }

    pub fn number_of_sets(n: i32, k: i32) -> i32 {
        let n = (n + k - 1) as i64;
        let r = (2 * k) as i64;

        let mut numerator = 1;
        let mut denominator = 1;

        for i in 0..r {
            numerator = numerator * (n - i) % Self::MOD;
            denominator = denominator * (i + 1) % Self::MOD;
        }

        (numerator * Self::power(denominator, Self::MOD - 2) % Self::MOD) as i32
    }
}
