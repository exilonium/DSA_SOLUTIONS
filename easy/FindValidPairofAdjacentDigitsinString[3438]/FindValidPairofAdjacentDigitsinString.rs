impl Solution {
    pub fn find_valid_pair(s: String) -> String {
        let bytes = s.as_bytes();

        let mut freq = [0; 10]; // frequency table

        for &b in bytes {
            freq[(b - b'0') as usize] += 1;
        }

        // Check adjacent pairs
        for i in 0..bytes.len() - 1 {
            let a = (bytes[i] - b'0') as usize;
            let b = (bytes[i + 1] - b'0') as usize;

            if a != b && freq[a] == a && freq[b] == b {
                return s[i..i + 2].to_string();
            }
        }

        String::new()
    }
}
