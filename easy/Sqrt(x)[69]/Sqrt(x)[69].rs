//optimal
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x: i64 = x as i64;
        let mut low: i64 = 0;
        let mut high: i64 = x;

        while low <= high {
            let mut mid = low + (high - low) / 2;
            if mid * mid > x {
                high = mid - 1;
            } else if (mid * mid) < x {
                low = mid + 1;
            } else {
                return mid as _;
            }
        }
        return ((high + low) / 2) as i32;
    }
}
