// brut (or optimal since it beats 100% in runtime and 97.06% in memory)
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        for i in 0..=nums.len() {
            res += i as i32;
        }

        for i in nums.iter() {
            res -= i;
        }
        // you can replace the above loop code for this
        // res -= nums.iter().sum::<i32>();
        res
    }
}
