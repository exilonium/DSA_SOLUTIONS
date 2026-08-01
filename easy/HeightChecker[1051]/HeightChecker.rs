impl Solution {
    pub fn height_checker(heights: Vec<i32>) -> i32 {
        let mut sor = heights.clone();
        sor.sort_unstable();
        let mut count = 0;
        for i in 0..sor.len() {
            if sor[i] != heights[i] {
                count += 1;
            }
        }
        count
    }
}
