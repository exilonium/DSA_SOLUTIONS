impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        let mut start: i32 = 1;
        let mut end: i32 = num;

        while end >= start {
            // let mut mid = start/2 + end/2 + (start%2 + end%2)/2; // old trick to avoid overflow
            let mut mid = start + (end - start) / 2;
            if mid == num / mid && num % mid == 0 {
                return true;
            } else if mid > num / mid {
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }
        return false;
    }
}
