impl Solution {
    pub fn furthest_distance_from_origin(moves: String) -> i32 {
        let mut L: i32 = 0;
        let mut R: i32 = 0;
        let mut D: i32 = 0;
        for i in moves.chars() {
            match i {
                'L' => L += 1,
                'R' => R += 1,
                _ => D += 1,
            }
        }
        (L - R).abs() + D
    }
}
