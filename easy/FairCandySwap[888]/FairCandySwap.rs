use std::collections::HashSet;

impl Solution {
    pub fn fair_candy_swap(alice_sizes: Vec<i32>, bob_sizes: Vec<i32>) -> Vec<i32> {
        let sum_a: i32 = alice_sizes.iter().sum();
        let sum_b: i32 = bob_sizes.iter().sum();

        let diff = (sum_b - sum_a) / 2;

        let bob: HashSet<i32> = bob_sizes.into_iter().collect();

        for x in alice_sizes {
            let y = x + diff;

            if bob.contains(&y) {
                return vec![x, y];
            }
        }

        vec![]
    }
}
