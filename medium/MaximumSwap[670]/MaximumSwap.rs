impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits = num.to_string().into_bytes();

        let mut last = [0usize; 10];

        for i in 0..digits.len() {
            let d = (digits[i] - b'0') as usize;
            last[d] = i;
        }

        for i in 0..digits.len() {
            let current = (digits[i] - b'0') as usize;

            for d in (current + 1..10).rev() {
                if last[d] > i {
                    digits.swap(i, last[d]);

                    let mut result = 0;

                    for digit in digits {
                        result = result * 10 + (digit - b'0') as i32;
                    }

                    return result;
                }
            }
        }

        num
    }
}
