impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|mut x| {
                let mut s = 0;
                while x > 0 {
                    s += x % 10;
                    x /= 10;
                }
                s
            })
            .min()
            .unwrap()
    }
}
