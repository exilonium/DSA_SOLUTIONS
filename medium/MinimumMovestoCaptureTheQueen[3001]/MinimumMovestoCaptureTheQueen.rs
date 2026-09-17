impl Solution {
    pub fn min_moves_to_capture_the_queen(a: i32, b: i32, c: i32, d: i32, e: i32, f: i32) -> i32 {
        // Rook captures queen
        if a == e {
            if !(c == a && d > b.min(f) && d < b.max(f)) {
                return 1;
            }
        }

        if b == f {
            if !(d == b && c > a.min(e) && c < a.max(e)) {
                return 1;
            }
        }

        // Bishop captures queen
        if (c - e).abs() == (d - f).abs() {
            // Rook blocks bishop only if it is on the same
            // diagonal AND between bishop and queen.
            if !((a - c).abs() == (b - d).abs()
                && (a > c.min(e) && a < c.max(e))
                && (b > d.min(f) && b < d.max(f)))
            {
                return 1;
            }
        }

        2
    }
}
