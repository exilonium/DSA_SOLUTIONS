impl Solution {
    pub fn earliest_finish_time(
        land_start_time: Vec<i32>,
        land_duration: Vec<i32>,
        water_start_time: Vec<i32>,
        water_duration: Vec<i32>,
    ) -> i32 {
        fn solve(start1: &[i32], dur1: &[i32], start2: &[i32], dur2: &[i32]) -> i32 {
            let mut earliest_finish = i32::MAX;

            for i in 0..start1.len() {
                earliest_finish = earliest_finish.min(start1[i] + dur1[i]);
            }

            let mut ans = i32::MAX;

            for i in 0..start2.len() {
                ans = ans.min(earliest_finish.max(start2[i]) + dur2[i]);
            }

            ans
        }

        solve(
            &land_start_time,
            &land_duration,
            &water_start_time,
            &water_duration,
        )
        .min(solve(
            &water_start_time,
            &water_duration,
            &land_start_time,
            &land_duration,
        ))
    }
}
