impl Solution {
    pub fn count_points(rings: String) -> i32 {
        let mut rods = [0u8; 10];

        for pair in rings.as_bytes().chunks_exact(2) {
            let bit = match pair[0] {
                b'R' => 1,
                b'G' => 2,
                _ => 4,
            };

            rods[(pair[1] - b'0') as usize] |= bit;
        }

        rods.into_iter().filter(|&mask| mask == 7).count() as i32
    }
}
