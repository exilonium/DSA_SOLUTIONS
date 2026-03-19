impl Solution {
    pub fn check_if_pangram(sentence: String) -> bool {
        let mut res = vec![0; 26];
        for ch in sentence.bytes() {
            let idx = ch as u32 - 97; // basically subtracting 'a' as u32
            if idx >= 0 && idx <= 26 {
                res[idx as usize] = 1_i32;
            }
        }
        res.iter().sum::<i32>() == 26
    }
}
