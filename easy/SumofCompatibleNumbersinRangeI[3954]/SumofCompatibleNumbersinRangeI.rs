impl Solution {
    pub fn sum_of_good_integers(n: i32, k: i32) -> i32 {
        let mut res = 0;
        for x in (n - k).max(1)..=(n + k) {
            if (n & x) == 0 {
                res += x;
            }
        }
        res
    }
}
