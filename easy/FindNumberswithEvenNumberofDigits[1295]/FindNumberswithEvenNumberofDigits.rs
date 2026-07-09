impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let mut even_digits = true;
        let mut res = 0;
        for i in nums {
            let mut n = i;
            while n != 0 {
                n = n / 10;
                even_digits = !even_digits;
            }
            if even_digits {
                res += 1;
            }
            even_digits = true;
        }
        res
    }
}
