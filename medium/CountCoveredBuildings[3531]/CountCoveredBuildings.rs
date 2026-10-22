impl Solution {
    pub fn count_covered_buildings(n: i32, buildings: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;

        let mut min_x = vec![i32::MAX; n + 1];
        let mut max_x = vec![i32::MIN; n + 1];
        let mut min_y = vec![i32::MAX; n + 1];
        let mut max_y = vec![i32::MIN; n + 1];

        for b in &buildings {
            let (x, y) = (b[0], b[1]);

            min_x[y as usize] = min_x[y as usize].min(x);
            max_x[y as usize] = max_x[y as usize].max(x);

            min_y[x as usize] = min_y[x as usize].min(y);
            max_y[x as usize] = max_y[x as usize].max(y);
        }

        let mut ans = 0;

        for b in &buildings {
            let (x, y) = (b[0], b[1]);
            let (xi, yi) = (x as usize, y as usize);

            if min_x[yi] < x && x < max_x[yi] && min_y[xi] < y && y < max_y[xi] {
                ans += 1;
            }
        }

        ans
    }
}
