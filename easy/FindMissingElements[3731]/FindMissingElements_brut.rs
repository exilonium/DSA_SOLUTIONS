impl Solution {
    pub fn find_missing_elements(nums: Vec<i32>) -> Vec<i32> {
        let mut sor = nums.clone();
        sor.sort_unstable();

        let mut res = Vec::new();
        let mut count = sor[0];
        for i in 1..sor.len() {
            count += 1;
            while count != sor[i] {
                res.push(count);
                count += 1;
            }
        }
        res
    }
}
