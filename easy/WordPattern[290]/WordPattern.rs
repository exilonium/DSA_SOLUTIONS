impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        if pattern.len() != s.matches(' ').count() + 1 {
            return false;
        }
        let mut mappings = [None; 26];
        for (c, word) in pattern.bytes().zip(s.split(' ')) {
            let index = (c - b'a') as usize;
            if let Some(mapped_word) = mappings[index] {
                if word != mapped_word {
                    return false;
                }
            } else {
                if mappings.contains(&Some(word)) {
                    return false;
                }
                mappings[index] = Some(word);
            }
        }
        true
    }
}
