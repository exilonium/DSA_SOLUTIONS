impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        let mut len = 0;
        let mut comma = 0;
        let mut start = 1000;

        if n < start {
            return 0;
        }
        while n >= start {
            comma += (n - start) + 1;
            start *= 1000;
        }
        comma
    }
}
