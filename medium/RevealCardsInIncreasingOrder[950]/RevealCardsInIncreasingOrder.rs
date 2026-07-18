use std::collections::VecDeque;

impl Solution {
    pub fn deck_revealed_increasing(mut deck: Vec<i32>) -> Vec<i32> {
        deck.sort();

        let mut dq = VecDeque::new();

        for &card in deck.iter().rev() {
            if let Some(last) = dq.pop_back() {
                dq.push_front(last);
            }
            dq.push_front(card);
        }

        dq.into_iter().collect()
    }
}
