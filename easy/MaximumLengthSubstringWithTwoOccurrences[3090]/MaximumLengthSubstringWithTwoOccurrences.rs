impl Solution {
    pub fn maximum_length_substring(s: String) -> i32 {
        let bytes = s.as_bytes();
        let mut counts = [0usize; 26];
        let mut left = 0usize;
        let mut best = 0usize;

        for right in 0..bytes.len() {
            counts[(bytes[right] - b'a') as usize] += 1;

            while counts[(bytes[right] - b'a') as usize] > 2 {
                counts[(bytes[left] - b'a') as usize] -= 1;
                left += 1;
            }

            best = best.max(right - left + 1);
        }

        best as i32
    }
}
