impl Solution {
    pub fn minimum_distance(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        let mut first = vec![-1i32; n + 1];
        let mut second = vec![-1i32; n + 1];

        let mut ans = i32::MAX;

        for (i, &x) in nums.iter().enumerate() {
            let x = x as usize;

            if second[x] != -1 {
                // first[x], second[x], i form 3 consecutive
                // occurrences of x.
                ans = ans.min(2 * (i as i32 - first[x]));

                // Slide the window.
                first[x] = second[x];
                second[x] = i as i32;
            } else if first[x] != -1 {
                second[x] = i as i32;
            } else {
                first[x] = i as i32;
            }
        }

        if ans == i32::MAX { -1 } else { ans }
    }
}
