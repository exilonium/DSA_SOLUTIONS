impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let size = nums.len();
        if size <= 1 {
            return 0;
        }

        let sum: i32 = nums.iter().sum();

        if sum % 2 != 0 {
            return 0;
        }

        (size - 1) as i32
    }
}
