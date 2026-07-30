impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut res = 0;
        for i in 0..word.len() {
            res += (i / 8) + 1;
        }
        res as _
    }
}
