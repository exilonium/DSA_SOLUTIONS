impl Solution {
    pub fn circular_array_loop(nums: Vec<i32>) -> bool {
        let n = nums.len();

        let next = |i: usize, forward: bool| -> Option<usize> {
            if (nums[i] > 0) != forward {
                return None;
            }

            let j = (i as i32 + nums[i]).rem_euclid(n as i32) as usize;

            if j == i {
                return None;
            }

            Some(j)
        };

        for i in 0..n {
            let forward = nums[i] > 0;
            let mut slow = i;
            let mut fast = i;

            loop {
                slow = match next(slow, forward) {
                    Some(j) => j,
                    None => break,
                };

                fast = match next(fast, forward) {
                    Some(j) => j,
                    None => break,
                };

                fast = match next(fast, forward) {
                    Some(j) => j,
                    None => break,
                };

                if slow == fast {
                    return true;
                }
            }
        }

        false
    }
}
