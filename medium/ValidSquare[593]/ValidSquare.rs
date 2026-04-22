impl Solution {
    pub fn valid_square(p1: Vec<i32>, p2: Vec<i32>, p3: Vec<i32>, p4: Vec<i32>) -> bool {
        fn dist(a: &Vec<i32>, b: &Vec<i32>) -> i32 {
            (a[0] - b[0]).pow(2) + (a[1] - b[1]).pow(2)
        }

        let mut d = vec![
            dist(&p1, &p2),
            dist(&p1, &p3),
            dist(&p1, &p4),
            dist(&p2, &p3),
            dist(&p2, &p4),
            dist(&p3, &p4),
        ];

        d.sort_unstable(); // this makes it all work

        // 4 equal sides, 2 equal diagonals, and non-zero side
        d[0] > 0 && d[0] == d[1] && d[1] == d[2] && d[2] == d[3] && d[4] == d[5]
    }
}
