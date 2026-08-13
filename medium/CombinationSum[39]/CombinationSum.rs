impl Solution {
    fn combinations(ans: &mut Vec<Vec<i32>>, buffer: &mut Vec<i32>, candidates: &[i32], sum: i32) {
        if sum == 0 {
            ans.push(buffer.clone());
        } else if sum > 0 {
            for i in 0..candidates.len() {
                buffer.push(candidates[i]);
                Solution::combinations(ans, buffer, &candidates[i..], sum - candidates[i]);
                buffer.pop();
            }
        }
    }

    pub fn combination_sum(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut ans: Vec<Vec<i32>> = vec![];
        let mut buffer: Vec<i32> = vec![];
        let mut candidates: Vec<i32> = candidates.clone();
        candidates.sort();
        Solution::combinations(&mut ans, &mut buffer, &candidates, target);

        ans
    }
}
