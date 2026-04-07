// O(1) no hashmap lookup Solution
impl Solution {
    pub fn get_hint(secret: String, guess: String) -> String {
        let mut bulls = 0;
        let mut cows = 0;
        let mut count = [0; 10]; // storing the digits as index

        for (s, g) in secret.bytes().zip(guess.bytes()) {
            if s == g {
                bulls += 1;
            } else {
                let si = (s - b'0') as usize; // getting the index of s
                let gi = (g - b'0') as usize; // similiarly getting the index of g

                if count[si] < 0 {
                    cows += 1;
                }
                if count[gi] > 0 {
                    cows += 1;
                }

                count[si] += 1;
                count[gi] -= 1;
            }
        }

        format!("{bulls}A{cows}B")
    }
}
