impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for i in nums {
            let mut num = i;
            let mut tmp = vec![];
            while num != 0 {
                tmp.push(num % 10);
                num = num / 10;
            }
            for i in tmp.into_iter().rev() {
                res.push(i);
            }
        }
        res
    }
}
