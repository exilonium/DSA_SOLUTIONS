impl Solution {
    pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_unstable();

        for i in 1..nums.len() {
            nums[i] += nums[i - 1];
        }

        queries
            .iter()
            .map(|&q| {
                // Approach 1: partition_point
                nums.partition_point(|&sum| sum <= q) as i32

                // Approach 2: Manual binary search

                /*
                let (mut l, mut r) = (0, nums.len());

                while l < r {
                    let mid = l + (r - l) / 2;

                    if nums[mid] <= q {
                        l = mid + 1;
                    } else {
                        r = mid;
                    }
                }

                l as i32
                */
            })
            .collect()
    }
}
