impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut sum = 0;
        let mut mul = 1;
        let mut cur = n;
        while cur > 0 {
            let d = cur % 10;
            sum += d;
            mul *= d;
            cur /= 10;
        }
        n % (sum + mul) == 0
    }
}
