impl Solution {
    pub fn concat_hex36(n: i32) -> String {
        fn convert(mut n: i32, base: i32) -> String {
            let mut ans = String::new();
            let digits = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

            while n > 0 {
                ans.push(digits[(n % base) as usize] as char);
                n /= base;
            }

            ans.chars().rev().collect()
        }

        convert(n * n, 16) + &convert(n * n * n, 36)
    }
}
