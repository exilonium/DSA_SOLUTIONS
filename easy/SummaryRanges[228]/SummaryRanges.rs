// absolute messy code (my one time pass)
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        if nums.is_empty() {
            return res;
        }
        let mut count = nums[0];
        let mut start = count;
        let mut prev = 0;
        for i in nums {
            if i != count {
                if start == prev {
                    res.push(start.to_string());
                } else {
                    let elem = String::from(format!("{}->{}", start, prev));
                    res.push(elem);
                }
                count = i;
                start = count;
            }
            count += 1;

            prev = i;
        }
        if start == prev {
            res.push(start.to_string());
        } else {
            let elem = String::from(format!("{}->{}", start, prev));
            res.push(elem);
        }
        res
    }
}
