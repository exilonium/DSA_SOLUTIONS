impl Solution {
    pub fn mirror_distance(n: i32) -> i32 {
        let mut x = n;
        let mut rev = 0;
        while x != 0 {
            let tmp = x % 10;
            x = x / 10;
            rev = rev * 10 + tmp;
        }
        (rev - n).abs()
    }
}
