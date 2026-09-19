impl Solution {
    pub fn smallest_index(nums: Vec<i32>) -> i32 {
        let mut idx = 0;

        for mut i in nums {
            let mut sum = 0;

            while i > 0 && sum <= idx {
                sum += i % 10;
                i /= 10;
            }

            if sum == idx {
                return sum;
            }
            idx += 1;
        }

        -1
    }
}
