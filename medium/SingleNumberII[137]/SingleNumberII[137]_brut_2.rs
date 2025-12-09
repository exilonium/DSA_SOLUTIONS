// brut 2
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut result = 0;

        for bit in 0..32 {
            let mut count = 0;
            for &num in &nums {
                if (num >> bit) & 1 == 1 {
                    count += 1;
                }
            }

            if count % 3 == 1 {
                result |= 1 << bit;
            }
        }

        result
    }
}
