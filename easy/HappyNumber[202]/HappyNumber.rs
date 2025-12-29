impl Solution {
    fn next(mut n: i32) -> i32 {
        let mut res = 0;
        while n > 0 {
            let d = n % 10;
            res += d * d;
            n /= 10;
        }
        res
    }

    pub fn is_happy(n: i32) -> bool {
        let mut slow = n;
        let mut fast = Self::next(n);
        while fast != 1 && slow != fast {
            slow = Self::next(slow);
            fast = Self::next(Self::next(fast));
        }
        return fast == 1;
    }
}
