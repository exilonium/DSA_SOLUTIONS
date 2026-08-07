// optimal
use std::collections::VecDeque;

impl Solution {
    pub fn final_string(s: String) -> String {
        let mut dq = VecDeque::new();
        let mut rev = false;

        for c in s.chars() {
            if c == 'i' {
                rev = !rev;
            } else if rev {
                dq.push_front(c);
            } else {
                dq.push_back(c);
            }
        }

        if rev {
            dq.iter().rev().collect()
        } else {
            dq.iter().collect()
        }
    }
}
