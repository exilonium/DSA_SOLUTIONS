// 0ms
impl Solution {
    pub fn super_pow(a: i32, b: Vec<i32>) -> i32 {
        const MOD: i32 = 1337;

        // helper function: computes (x^n) % MOD
        fn mod_pow(mut x: i32, mut n: i32) -> i32 {
            let mut res = 1;
            x %= MOD;
            while n > 0 {
                res = (res * x) % MOD;
                n -= 1;
            }
            res
        }
        fn solve(a: i32, b: &[i32]) -> i32 {
            if b.is_empty() {
                return 1; // a^0 = 1
            }

            let last = b[b.len() - 1];
            let rest = &b[..b.len() - 1];

            // part1 -> (a^(rest))^10
            let rest = mod_pow(solve(a, rest), 10);

            // part2 -> a^(last_digit)
            let last = mod_pow(a, last);

            (rest * last) % MOD
        }

        solve(a % MOD, &b)
    }
}
