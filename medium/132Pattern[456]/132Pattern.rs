impl Solution {
    pub fn find132pattern(nums: Vec<i32>) -> bool {
        let mut stack: Vec<i32> = Vec::new();
        let mut third = i32::MIN;

        for &num in nums.iter().rev() {
            if num < third {
                return true;
            }
            while let Some(&top) = stack.last() {
                if top < num {
                    third = stack.pop().unwrap();
                } else {
                    break;
                }
            }
            stack.push(num);
        }
        false
    }
}
