// my Solution 0ms
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let mut cap = 0;
        let mut size = word.len();
        for i in word.chars() {
            let asci = i as u32;
            if asci <= 94 {
                cap += 1;
            }
        }
        if cap == size || cap == 0 {
            return true;
        }
        if cap > 1 && cap != size {
            return false;
        }
        if cap == 1 && size > 1 {
            if (word.chars().next().unwrap() as u32) < 94 {
                return true;
            }
        }
        return false;
    }
}
