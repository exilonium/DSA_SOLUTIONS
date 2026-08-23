impl Solution {
    pub fn sum_game(num: String) -> bool {
        let n = num.len();
        let bytes = num.as_bytes();

        let mut left_sum = 0i32;
        let mut right_sum = 0i32;
        let mut left_q = 0i32;
        let mut right_q = 0i32;

        for i in 0..n / 2 {
            if bytes[i] == b'?' {
                left_q += 1;
            } else {
                left_sum += (bytes[i] - b'0') as i32;
            }
        }

        for i in n / 2..n {
            if bytes[i] == b'?' {
                right_q += 1;
            } else {
                right_sum += (bytes[i] - b'0') as i32;
            }
        }

        // Odd number of '?' -> Alice wins.
        if (left_q + right_q) % 2 == 1 {
            return true;
        }

        // Bob can win only if the existing difference
        // can exactly be compensated by the '?'s.
        left_sum - right_sum != 9 * (right_q - left_q) / 2
    }
}
