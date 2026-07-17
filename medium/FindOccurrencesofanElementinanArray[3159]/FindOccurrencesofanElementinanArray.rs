impl Solution {
    pub fn occurrences_of_element(nums: Vec<i32>, queries: Vec<i32>, x: i32) -> Vec<i32> {
        let mut pos = Vec::new();

        for (i, &num) in nums.iter().enumerate() {
            if num == x {
                pos.push(i as i32);
            }
        }

        let mut ans = Vec::new();

        for k in queries {
            if (k as usize) <= pos.len() {
                ans.push(pos[(k - 1) as usize]);
            } else {
                ans.push(-1);
            }
        }

        ans
    }
}
