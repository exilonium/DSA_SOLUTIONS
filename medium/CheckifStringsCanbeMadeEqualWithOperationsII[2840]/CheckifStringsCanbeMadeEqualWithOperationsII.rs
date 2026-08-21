impl Solution {
    pub fn check_strings(s1: String, s2: String) -> bool {
        let mut freq1 = [[0; 26]; 2];
        let mut freq2 = [[0; 26]; 2];

        for (i, c) in s1.bytes().enumerate() {
            freq1[i % 2][(c - b'a') as usize] += 1;
        }

        for (i, c) in s2.bytes().enumerate() {
            freq2[i % 2][(c - b'a') as usize] += 1;
        }

        freq1 == freq2
    }
}
