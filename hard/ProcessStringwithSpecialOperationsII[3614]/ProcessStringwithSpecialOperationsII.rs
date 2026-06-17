impl Solution {
    pub fn process_str(s: String, k: i64) -> char {
        let mut res = String::new();
        let mut l = 0;
        let mut k = k;
        for ch in s.chars() {
            match ch {
                '*' => l = (l - 1).max(0),
                '#' => l *= 2,
                '%' => {}
                _ => l += 1,
            }
        }
        // now ive found the length of the final res string
        if k >= l {
            return '.';
        }

        let characters: Vec<char> = s.chars().collect();
        for &ch in characters.iter().rev() {
            match ch {
                '*' => l += 1,
                '#' => {
                    l /= 2;
                    k %= l;
                }
                '%' => k = l - k - 1,
                ch => {
                    l -= 1;
                    if k == l {
                        return ch;
                    }
                }
            }
        }
        '.'
    }
}
