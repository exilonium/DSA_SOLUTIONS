// optimal
impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut count = 0;
        for i in 0..strs[0].len() {
            for idx in 0..strs.len() - 1 {
                if &strs[idx][i..i + 1] > &strs[idx + 1][i..i + 1] {
                    count += 1;
                    break;
                }
            }
        }
        count
    }
}
