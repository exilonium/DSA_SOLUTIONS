impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut lower = 0u32;
        let mut upper = 0u32;

        for &b in word.as_bytes() {
            match b {
                b'a'..=b'z' => lower |= 1 << (b - b'a'),
                b'A'..=b'Z' => upper |= 1 << (b - b'A'),
                _ => {}
            }
        }

        (lower & upper).count_ones() as _
    }
}
