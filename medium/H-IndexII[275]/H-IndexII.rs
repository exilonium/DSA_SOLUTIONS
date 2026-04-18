impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let (mut l, mut r) = (0, citations.len() - 1);
        while l < r {
            let mut m = (l + r) / 2;
            if citations[m] < (citations.len() - m) as i32 {
                l = m + 1;
            } else {
                r = m;
            }
        }
        if citations[l] >= (citations.len() - l) as i32 {
            (citations.len() - l) as i32
        } else {
            0
        }
    }
}
