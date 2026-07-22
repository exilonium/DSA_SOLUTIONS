impl Solution {
    pub fn check_good_integer(n: i32) -> bool {
        let mut x = n;
        let mut digsum = 0;
        let mut sqsum = 0;
        while x != 0 {
            let temp = x % 10;
            x = x / 10;
            digsum += temp;
            sqsum += temp * temp;
        }
        (sqsum - digsum) >= 50
    }
}
