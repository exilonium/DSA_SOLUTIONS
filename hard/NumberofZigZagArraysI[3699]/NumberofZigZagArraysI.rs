impl Solution {
    pub fn zig_zag_arrays(n: i32, l: i32, r: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;

        let m = (r - l + 1) as usize;

        let mut up = vec![1i64; m];
        let mut down = vec![1i64; m];

        for _ in 0..(n - 1) {
            let mut prefix = vec![0i64; m + 1];
            let mut suffix = vec![0i64; m + 1];

            for i in 0..m {
                prefix[i + 1] = (prefix[i] + down[i]) % MOD;
            }

            for i in (0..m).rev() {
                suffix[i] = (suffix[i + 1] + up[i]) % MOD;
            }

            let mut next_up = vec![0i64; m];
            let mut next_down = vec![0i64; m];

            for i in 0..m {
                next_up[i] = prefix[i];
                next_down[i] = suffix[i + 1];
            }

            up = next_up;
            down = next_down;
        }

        ((up.iter().sum::<i64>() + down.iter().sum::<i64>()) % MOD) as i32
    }
}
