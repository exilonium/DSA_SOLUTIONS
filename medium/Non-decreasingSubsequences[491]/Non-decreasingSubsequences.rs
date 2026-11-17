impl Solution {
    pub fn find_subsequences(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut path = Vec::new();

        fn dfs(nums: &[i32], start: usize, path: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if path.len() >= 2 {
                ans.push(path.clone());
            }

            // nums[i] is in [-100, 100]
            let mut used = [false; 201];

            for i in start..nums.len() {
                if !path.is_empty() && nums[i] < *path.last().unwrap() {
                    continue;
                }

                let idx = (nums[i] + 100) as usize;

                if used[idx] {
                    continue;
                }

                used[idx] = true;
                path.push(nums[i]);

                dfs(nums, i + 1, path, ans);

                path.pop();
            }
        }

        dfs(&nums, 0, &mut path, &mut ans);
        ans
    }
}
