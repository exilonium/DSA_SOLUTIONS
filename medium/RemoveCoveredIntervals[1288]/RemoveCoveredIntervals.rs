impl Solution {
    pub fn remove_covered_intervals(mut intervals: Vec<Vec<i32>>) -> i32 {
        intervals.sort_unstable_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1]))); // soting by comparing a[0] and b[0] and then a[1] and b[1]

        let mut removed = 0;
        let mut max_end = 0;

        for interval in intervals.iter() {
            if interval[1] <= max_end {
                removed += 1;
            } else {
                max_end = interval[1];
            }
        }

        (intervals.len() - removed) as i32
    }
}
