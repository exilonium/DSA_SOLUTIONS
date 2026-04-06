struct NumArray {
    prev_sum : Vec<i32>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumArray {

    fn new(nums: Vec<i32>) -> Self {
        let size = nums.len();
        let mut prev_sum = vec![0;size+1];
        let mut prev = 0;
        for i in 0..size{
            prev = prev+nums[i];
            prev_sum[i+1] = prev;
        }
        Self { prev_sum }
    }
    
    fn sum_range(&self, left: i32, right: i32) -> i32 {
        self.prev_sum[right as usize +1] - self.prev_sum[left as usize] // trick fahh
    }
}

/**
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(left, right);
 */
