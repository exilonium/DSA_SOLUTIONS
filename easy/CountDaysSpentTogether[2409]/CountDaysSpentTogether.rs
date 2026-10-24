impl Solution {
    pub fn count_days_together(
        arrive_alice: String,
        leave_alice: String,
        arrive_bob: String,
        leave_bob: String,
    ) -> i32 {
        fn convert(date: &str) -> i32 {
            let days = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334];

            let month: usize = date[0..2].parse().unwrap();
            let day: i32 = date[3..5].parse().unwrap();

            days[month - 1] + day
        }

        let start = convert(&arrive_alice).max(convert(&arrive_bob));
        let end = convert(&leave_alice).min(convert(&leave_bob));

        (end - start + 1).max(0)
    }
}
