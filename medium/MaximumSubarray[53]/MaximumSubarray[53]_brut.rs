// this is a brut Solution this will prolly not pass through all the test cases
// but its a valid code

impl Solution {
    pub fn max_sub_array(nums: Vec<i32>) -> i32 {
        let mut max = i32::MIN;

        for i in 0..nums.len(){
            for j in i..nums.len(){
                let mut total = 0;
                for k in i..=j{
                    total +=nums[k];

                }
                max = std::cmp::max(total,max);
            }
        }
        return max;

    }
}
