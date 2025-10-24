// this is better than brut but it will still fail in leetcode cause its no better!

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;

        for i in 0..nums.len(){
            let mut total = 0;
            for j in i..nums.len(){
                total +=nums[j];
                max = std::cmp::max(total,max);
            }
        }
        return max;

    }
}
