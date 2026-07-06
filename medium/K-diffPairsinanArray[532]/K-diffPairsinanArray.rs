use std::collections::HashMap;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        // due to constraints k being positive
        // if k < 0{
        //     return 0;
        // }
        let mut map = HashMap::new();
        for num in &nums {
            map.entry(num).and_modify(|x| *x += 1).or_insert(1);
        }
        let mut res = 0;
        for &key in map.keys() {
            if let Some(&count) = map.get(&(key + k)) {
                if k != 0 || count > 1 {
                    res += 1;
                }
            }
        }
        res
    }
}
