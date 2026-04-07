// my approach using hashmap 0ms
// good
use std::collections::HashMap;
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let size = secret.len();
        let mut bulls = 0;
        let mut cows = 0;
        let mut map = HashMap::new();
        for c in secret.chars() {
            map.entry(c).and_modify(|count| *count += 1).or_insert(1);
        }
        for (s, g) in secret.chars().zip(guess.chars()) {
            if s == g {
                bulls += 1;
            }
            if let Some(count) = map.get_mut(&g) {
                if *count > 0 {
                    cows += 1;
                    *count -= 1;
                }
            }
        }
        cows -= bulls;
        format!("{bulls}A{cows}B")
    }
}
