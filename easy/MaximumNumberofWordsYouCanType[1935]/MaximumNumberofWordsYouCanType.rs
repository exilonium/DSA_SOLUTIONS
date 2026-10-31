impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let mut count = 0;

        for word in text.split_whitespace() {
            let mut valid = true;

            for c in word.chars() {
                if broken_letters.contains(c) {
                    valid = false;
                    break;
                }
            }

            if valid {
                count += 1;
            }
        }

        count
    }
}
