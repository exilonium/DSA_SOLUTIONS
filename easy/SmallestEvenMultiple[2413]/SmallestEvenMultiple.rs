impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n & 1 == 1 {
            // basically its n%2 == 1 ( if n is odd )
            return 2 * n;
        }
        n
    }
}
