impl Solution {
    pub fn total_hamming_distance(nums: Vec<i32>) -> i32 {
        let mut total = 0;
        let n = nums.len();

        for i in 0..30 {
            let mut count_ones = 0;
            for &num in &nums {
                if (num >> i) & 1 == 1 {
                    count_ones += 1;
                }
                // for above 2 lines we can also use .count_ones()
            }
            let count_zeros = n - count_ones;
            total += count_ones * count_zeros;
        }

        total as i32
    }
}
