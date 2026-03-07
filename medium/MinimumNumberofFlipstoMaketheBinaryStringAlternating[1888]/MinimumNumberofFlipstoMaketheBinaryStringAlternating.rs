// one of the hardest questien i did till now
// not my Solution
// tho i like the approach
impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let mut chars: Vec<char> = s.chars().collect();

        let mut chars_0 = vec![];
        let mut chars_1 = vec![];

        for i in 0..n * 2 {
            if i % 2 == 0 {
                chars_0.push('0');
                chars_1.push('1');
            } else {
                chars_0.push('1');
                chars_1.push('0');
            }
        }
        // until above we made two lists with the alternating 0101 and 1010 upto n length

        let mut res = n * 2;
        let (mut cnt_0, mut cnt_1) = (0, 0);

        let mut l = 0;
        for r in 0..n * 2 {
            if chars[r % n] != chars_0[r] {
                cnt_0 += 1
            }
            if chars[r % n] != chars_1[r] {
                cnt_1 += 1
            }

            if (r - l + 1) > n {
                if chars[l % n] != chars_0[l] {
                    cnt_0 -= 1
                }
                if chars[l % n] != chars_1[l] {
                    cnt_1 -= 1
                }
                l += 1;
            }

            if (r - l + 1) == n {
                res = res.min(cnt_0).min(cnt_1);
            }
        }
        res as i32
    }
}
