impl Solution {
    pub fn shortest_beautiful_substring(s: String, k: i32) -> String {
        let bytes = s.as_bytes();

        let mut ones = Vec::new();

        for i in 0..bytes.len() {
            if bytes[i] == b'1' {
                ones.push(i);
            }
        }

        if ones.len() < k as usize {
            return String::new();
        }

        let k = k as usize;

        let mut best_l = ones[0];
        let mut best_r = ones[k - 1];

        for i in 0..=ones.len() - k {
            let l = ones[i];
            let r = ones[i + k - 1];

            let len = r - l + 1;
            let best_len = best_r - best_l + 1;

            if len < best_len || (len == best_len && bytes[l..=r] < bytes[best_l..=best_r]) {
                best_l = l;
                best_r = r;
            }
        }

        s[best_l..=best_r].to_string()
    }
}
