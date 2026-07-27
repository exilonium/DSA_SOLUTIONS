impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let (mut first, mut second) = (0, 0);

        for x in nums {
            if x >= first {
                second = first;
                first = x;
            } else if x > second {
                second = x;
            }
        }

        (first - 1) * (second - 1)
    }
}
