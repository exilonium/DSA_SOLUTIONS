// 0 ms but not the optimal O(n log(n))
impl Solution {
    fn count_one(n: i32) -> i32 {
        let mut count = 0;
        let mut n = n;
        while n != 0 {
            count += n & 1;
            n = n >> 1;
        }
        count
    }
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n: usize = n as usize;
        let mut res = vec![1; n + 1]; // intializing the vector with defalut value 1 and size n+1
        for i in 0..n + 1 {
            res[i] = Self::count_one(i as i32);
        }
        res
    }
}
