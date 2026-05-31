impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut ans = 0;
        for i in 0..n - 1 {
            let x0 = points[i][0];
            let y0 = points[i][1];
            let x1 = points[i + 1][0];
            let y1 = points[i + 1][1];
            ans += std::cmp::max((x0 - x1).abs(), (y0 - y1).abs());
        }
        ans
    }
}
