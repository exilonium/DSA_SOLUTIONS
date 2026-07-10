impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut absent_count = 0;
        let mut late_count = 0;

        for char in s.chars() {
            match char {
                'L' => late_count += 1,
                'A' => {
                    late_count = 0;
                    absent_count += 1;
                }
                _ => late_count = 0,
            }

            if late_count == 3 || absentn_count == 2 {
                return false;
            }
        }

        true
    }
}
