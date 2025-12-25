// optimal
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if intervals.is_empty() {
            return vec![];
        }

        let mut res: Vec<Vec<i32>> = vec![];

        let mut intervals = intervals;
        intervals.sort_unstable();

        let mut last = intervals[0].clone();

        for item in intervals {
            if item[0] > last[1] {
                res.push(last);
                last = item;
            } else {
                last[1] = item[1].max(last[1]);
            }
        }
        res.push(last);

        res
    }
}
