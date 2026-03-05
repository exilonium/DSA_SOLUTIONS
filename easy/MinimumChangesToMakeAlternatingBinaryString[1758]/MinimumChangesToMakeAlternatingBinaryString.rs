impl Solution {
    pub fn min_operations(s: String) -> i32 {
        let res = s
            .bytes()
            .enumerate()
            .map(|(idx, elem)| (elem as i32 ^ idx as i32) & 1)
            .sum::<i32>();

        return res.min(s.len() as i32 - res);
    }
}
