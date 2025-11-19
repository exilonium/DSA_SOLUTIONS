// this is better than brut but not optimal..
use std::collections::{HashMap, BinaryHeap};

impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let k = k as usize;

        // 1. Count frequencies
        let mut freq = HashMap::new();
        for n in nums {
            *freq.entry(n).or_insert(0) += 1;
        }

        // 2. Min-heap of size K
        let mut heap = BinaryHeap::new();

        for (&num, &count) in &freq {
            // push negative count to simulate min-heap
            heap.push((-count, num));

            if heap.len() > k {
                heap.pop(); // remove element with lowest frequency
            }
        }

        // 3. Extract result
        heap.into_iter().map(|(_, num)| num).collect()
    }
}
