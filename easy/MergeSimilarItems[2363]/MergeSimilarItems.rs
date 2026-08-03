impl Solution {
    pub fn merge_similar_items(items1: Vec<Vec<i32>>, items2: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut cnt = [0; 1001];

        for item in items1 {
            cnt[item[0] as usize] += item[1];
        }

        for item in items2 {
            cnt[item[0] as usize] += item[1];
        }

        let mut ans = Vec::new();

        for (value, &weight) in cnt.iter().enumerate() {
            if weight > 0 {
                ans.push(vec![value as i32, weight]);
            }
        }

        ans
    }
}
