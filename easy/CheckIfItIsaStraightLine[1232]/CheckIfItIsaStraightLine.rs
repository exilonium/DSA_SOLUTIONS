impl Solution {
    pub fn check_straight_line(coordinates: Vec<Vec<i32>>) -> bool {
        let x1 = coordinates[0][0];
        let y1 = coordinates[0][1];
        let x2 = coordinates[1][0];
        let y2 = coordinates[1][1];

        let dx = x2 - x1;
        let dy = y2 - y1;

        coordinates.iter().skip(2).all(|p| {
            let dx2 = p[0] - x1;
            let dy2 = p[1] - y1;

            dy * dx2 == dx * dy2
        })
    }
}
