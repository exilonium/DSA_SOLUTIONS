impl Solution {
    pub fn shortest_completing_word(license_plate: String, words: Vec<String>) -> String {
        let mut need = [0; 26];

        for c in license_plate.chars() {
            if c.is_ascii_alphabetic() {
                need[c.to_ascii_lowercase() as usize - 'a' as usize] += 1;
            }
        }

        let mut ans = String::new();

        for word in words {
            let mut have = [0; 26];

            for c in word.chars() {
                have[c.to_ascii_lowercase() as usize - 'a' as usize] += 1;
            }

            let valid = (0..26).all(|i| have[i] >= need[i]);

            if valid && (ans.is_empty() || word.len() < ans.len()) {
                ans = word;
            }
        }

        ans
    }
}
