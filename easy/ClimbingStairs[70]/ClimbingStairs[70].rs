// basically answer is fibbo(n)
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n <= 3 {
            return n;
        }
        let mut prev1 = 3;
        let mut prev2 = 2;
        let mut cur = 0;
        for _ in 3..n {
            cur = prev1 + prev2;
            prev2 = prev1;
            prev1 = cur;
        }
        cur
    }
}
