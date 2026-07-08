// not my solution as this solution was really hard for me
impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        const MOD: i64 = 1_000_000_007;

        let n = s.len();
        let bytes = s.as_bytes();

        let mut a = vec![0i64; n + 1];
        let mut b = vec![0i64; n + 1];
        let mut len = vec![0usize; n + 1];
        let mut pow = vec![1i64; n + 1];

        for i in 1..=n {
            pow[i] = pow[i - 1] * 10 % MOD;
        }

        for i in 0..n {
            let d = (bytes[i] - b'0') as i64;
            a[i + 1] = a[i] + d;
            len[i + 1] = len[i] + (d > 0) as usize;
            b[i + 1] = if d == 0 { b[i] } else { (b[i] * 10 + d) % MOD };
        }

        queries
            .into_iter()
            .map(|q| {
                let l = q[0] as usize;
                let r = q[1] as usize + 1;

                let x = (b[r] - b[l] * pow[len[r] - len[l]] % MOD + MOD) % MOD;
                ((x * (a[r] - a[l])) % MOD) as i32
            })
            .collect()
    }
}
