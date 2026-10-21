impl Solution {
    pub fn count_rotations(s: String, k: i32) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        let mut ans = 0;

        for shift in 0..n {
            let mut score = 0;

            for i in 0..n - 1 {
                if chars[(i + shift) % n] == chars[(i + shift + 1) % n] {
                    score += 1;
                }
            }

            if score == k {
                ans += 1;
            }
        }

        ans
    }
}
