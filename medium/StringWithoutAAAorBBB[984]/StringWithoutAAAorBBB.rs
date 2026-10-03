impl Solution {
    pub fn str_without3a3b(mut a: i32, mut b: i32) -> String {
        let mut ans = String::new();

        while a > 0 || b > 0 {
            if a > b {
                ans.push('a');
                a -= 1;

                if a > b && a > 0 {
                    ans.push('a');
                    a -= 1;
                }

                if b > 0 {
                    ans.push('b');
                    b -= 1;
                }
            } else {
                ans.push('b');
                b -= 1;

                if b > a && b > 0 {
                    ans.push('b');
                    b -= 1;
                }

                if a > 0 {
                    ans.push('a');
                    a -= 1;
                }
            }
        }

        ans
    }
}
