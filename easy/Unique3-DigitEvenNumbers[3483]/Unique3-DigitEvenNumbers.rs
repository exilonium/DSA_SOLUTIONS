use std::collections::HashSet;

impl Solution {
    pub fn total_numbers(digits: Vec<i32>) -> i32 {
        let n = digits.len();
        let mut set = HashSet::new();

        for i in 0..n {
            // Hundreds digit cannot be zero
            if digits[i] == 0 {
                continue;
            }

            for j in 0..n {
                if j == i {
                    continue;
                }

                for k in 0..n {
                    if k == i || k == j {
                        continue;
                    }

                    if digits[k] % 2 != 0 {
                        continue;
                    }

                    let num = digits[i] * 100 + digits[j] * 10 + digits[k];

                    set.insert(num);
                }
            }
        }

        set.len() as i32
    }
}
