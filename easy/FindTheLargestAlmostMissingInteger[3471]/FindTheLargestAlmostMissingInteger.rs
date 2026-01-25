// not my Solution but damn its so neat
// i wish i could come up with such neat Solutions
// one of the optimal approach btw

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    pub fn largest_integer(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = -1;

        let mut map = HashMap::new();
        let mut window = VecDeque::new();

        let k = k as usize;

        'initialize_window: for i in 0..k {
            // Add to the initial window
            window.push_back(nums[i]);

            // always set to 1, even if duplicates occur,
            // because this loop initializes 1
            // k-sized sliding window.
            map.insert(nums[i], 1);
        }

        'iterate_windows: for w in nums.windows(k + 1) {
            let new = w[k];

            // remove the left-most element
            // from the window
            window.pop_front();

            // Add the newest val to the window
            window.push_back(new);

            // For each number in the window,
            // increase the number of windows
            // it appears in by 1.
            //
            // Uses "handled" to avoid duplicate counting.
            let mut handled = HashSet::new();

            'count_num_appearances: for num in &window {
                if !handled.contains(num) {
                    map.entry(*num).and_modify(|e| *e += 1).or_insert(1);

                    handled.insert(*num);
                }
            }
        }

        // Find the highest value that appears in 1 window.
        for (k, v) in map.iter() {
            if *v == 1 {
                ans = ans.max(*k);
            }
        }

        ans
    }
}
