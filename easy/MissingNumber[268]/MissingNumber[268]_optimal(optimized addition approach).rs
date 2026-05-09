//using (n)*(n+1)/2 approach
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        let mut res: i32 = 0;
        //for i in 0..=nums.len(a) {
        //    res += i as i32;
        //}
        let size = nums.len();
        let tsum = ((size) * (size + 1) / 2) as i32; // sum upto n natural number (n)*(n+1)/2

        for i in nums.iter() {
            res += i;
        }
        // you can replace the above loop code for this
        // res -= nums.iter().sum::<i32>();
        tsum - res
    }
}
