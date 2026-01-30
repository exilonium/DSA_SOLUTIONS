impl Solution {
    pub fn min_eating_speed(piles: Vec<i32>, h: i32) -> i32 {
        let mut left = 1;
        let mut right = *piles.iter().max().unwrap();
        while left < right {
            let mid = (left + right) / 2;
            let mut hours = 0;
            for pile in &piles {
                hours += (*pile as f64 / mid as f64).ceil() as i32;
            }
            if hours <= h {
                right = mid;
            } else {
                left = mid + 1;
            }
        }
        right
    }
}
