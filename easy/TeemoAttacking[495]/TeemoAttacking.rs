impl Solution {
    pub fn find_poisoned_duration(time_series: Vec<i32>, duration: i32) -> i32 {
        let n = time_series.len();
        if n == 0 {
            return 0;
        }

        let mut res = 0;

        for i in 0..n - 1 {
            let diff = time_series[i + 1] - time_series[i];
            res += if diff > duration { duration } else { diff };
        }

        res + duration
    }
}
