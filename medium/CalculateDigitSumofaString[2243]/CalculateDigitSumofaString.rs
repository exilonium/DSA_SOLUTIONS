impl Solution {
    pub fn digit_sum(mut s: String, k: i32) -> String {
        let k = k as usize;

        while s.len() > k {
            let mut res = String::new();

            for chunk in s.as_bytes().chunks(k) {
                let sum: u32 = chunk.iter().map(|&c| (c - b'0') as u32).sum();

                res.push_str(&sum.to_string());
            }

            s = res;
        }

        s
    }
}
