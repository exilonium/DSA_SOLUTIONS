impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let n = s.len();
        let mut arr: Vec<char> = s.chars().collect();
        let mut counts = [0usize; 26];

        for i in 0..(n / 2) {
            counts[(arr[i] as u8 - b'a') as usize] += 1;
        }

        let mut idx = 0;
        for c in 0..26 {
            while counts[c] > 0 {
                arr[idx] = (b'a' + c as u8) as char;
                idx += 1;
                counts[c] -= 1;
            }
        }

        for i in 0..(n / 2) {
            arr[n - 1 - i] = arr[i];
        }

        arr.into_iter().collect()
    }
}
