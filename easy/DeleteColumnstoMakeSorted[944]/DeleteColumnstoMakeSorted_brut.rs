// brut force 14 ms
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..strs[0].len() {
            let mut prev = String::new();
            for idx in 0..strs.len() {
                let curr = &strs[idx][i..i + 1];
                if curr < prev.as_str() {
                    count += 1;
                    break;
                }
                prev = curr.to_string();
            }
        }
        count
    }
}
