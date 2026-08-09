impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        for x in n.. {
            // this is basically infinite loop
            let mut num = x;
            let mut product = 1;

            while num > 0 {
                product *= num % 10;
                num /= 10;
            }

            if product % t == 0 {
                return x;
            }
        }
        -1
    }
}
