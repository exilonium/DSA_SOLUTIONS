impl Solution {
    pub fn uniform_array(nums: Vec<i32>) -> bool {
        let min = match nums.iter().min() {
            Some(&x) => x,
            None => return true,
        };

        if min % 2 == 1 {
            return true;
        }

        nums.iter().all(|&x| x % 2 == 0)
    }
}
