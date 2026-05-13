impl Solution {
    pub fn reverse_only_letters(s: String) -> String {
        let mut s: Vec<char> = s.chars().collect();
        let (mut start, mut end) = (0, s.len() - 1);
        loop {
            while start < s.len() {
                if s[start].is_alphabetic() {
                    break;
                }
                start += 1;
            }
            // checking if underflow
            while end != std::usize::MAX {
                if s[end].is_alphabetic() {
                    break;
                }
                end -= 1;
            }
            if start >= s.len() || end == std::usize::MAX || start >= end {
                break;
            }
            s.swap(start, end);
            start += 1;
            end -= 1;
        }
        s.iter().collect::<String>()
    }
}
