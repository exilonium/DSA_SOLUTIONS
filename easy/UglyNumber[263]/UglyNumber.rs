impl Solution {
    pub fn is_ugly(n: i32) -> bool {
        if n == 0 {
            return false;
        }
        let div = [2, 3, 5];
        let mut num = n;
        for i in div {
            while num % i == 0 {
                num /= i;
            }
        }
        num == 1
    }
}
