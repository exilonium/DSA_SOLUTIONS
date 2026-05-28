impl Solution {
    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        for &l in letters.iter() {
            if l > target {
                return l;
            }
        }
        letters[0]
    }
}
