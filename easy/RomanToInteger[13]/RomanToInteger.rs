use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let map = HashMap::from([
            ('I', 1),
            ('V', 5),
            ('X', 10),
            ('L', 50),
            ('C', 100),
            ('D', 500),
            ('M', 1000),
        ]);

        let mut sum = 0;
        let mut prev = 0;

        for c in s.chars() {
            let val = map[&c];

            if val > prev {
                // undo previous added value and apply subtract rule
                sum += val - 2 * prev;
            } else {
                sum += val;
            }

            prev = val;
        }

        sum
    }
}
