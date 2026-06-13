impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut res = String::new();
        for word in words {
            let mut w = 0;
            for ch in word.chars() {
                let tmp = (ch as u8 - b'a');
                // same as above but hardcoded
                //let tmp = j as i32 - 97;
                w += weights[tmp as usize];
            }
            let letter = (b'a' + (25 - (w % 26)) as u8) as char;
            res.push(letter);
        }
        res
    }
}
