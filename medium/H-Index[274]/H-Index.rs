// not my Solution
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations;
        citations.sort_unstable();
        let n = citations.len();
        for i in (1..=n).rev() {
            let j = i as i32;
            if citations[n - 1] >= j && citations[n - i] >= j {
                return j;
            }
        }
        0
    }
}
