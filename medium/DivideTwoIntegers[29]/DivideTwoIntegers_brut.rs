// this Solution wont work out since its a really slow and brut
impl Solution {
    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        let mut count = 0;
        let mut divisor = divisor;
        let mut res = dividend;
        let mut neg = false;
        if divisor < 0 {
            neg = !neg;
            divisor = -divisor;
        }
        if res < 0 {
            neg = !neg;
            res = -res;
        }
        while divisor <= res {
            count += 1;
            res -= divisor;
        }
        if neg {
            count = -count;
        }
        count
    }
}
