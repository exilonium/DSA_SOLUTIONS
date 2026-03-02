impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut res = 0i32;
        let mut count = 0;
        let mut n = n;
        let mut foundone = false;
        while n > 0 {
            let last = n & 1;
            if last == 1 {
                foundone = true;
                res = res.max(count);
                count = 0;
            }
            if foundone {
                count += 1;
            }
            n >>= 1;
        }
        res
    }
}
