impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        // your distance to target
        let d = target[0].abs() + target[1].abs();
        ghosts
            .iter()
            .all(|g| (g[0] - target[0]).abs() + (g[1] - target[1]).abs() > d)
    }
}
