impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let size = nums.len();
        let mut res = vec![-1; size];
        let mut stack: Vec<usize> = Vec::new(); // store indices

        for i in 0..(2 * size) {
            let curr = nums[i % size];

            while let Some(&top) = stack.last() {
                if nums[top] < curr {
                    stack.pop();
                    res[top] = curr;
                } else {
                    break;
                }
            }

            if i < size {
                stack.push(i);
            }
        }

        res
    }
}
