impl Solution {
    fn helper(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            return Self::helper(n - 1) + Self::helper(n - 2);
        }
    }
    pub fn fib(n: i32) -> i32 {
        Self::helper(n)
    }
}
