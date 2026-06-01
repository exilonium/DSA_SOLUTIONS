impl Solution {
    pub fn minimum_cost(mut c: Vec<i32>) -> i32 {
        c.sort_unstable_by(|a, b| b.cmp(a));
        c.chunks(3).flat_map(|c| c.iter().take(2)).sum()
    }
}
