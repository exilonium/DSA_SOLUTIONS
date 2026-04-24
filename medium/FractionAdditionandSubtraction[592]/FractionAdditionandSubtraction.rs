impl Solution {
    pub fn fraction_addition(expression: String) -> String {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a.abs() } else { gcd(b, a % b) }
        }

        let chars: Vec<char> = expression.chars().collect();
        let mut i = 0;

        let mut num = 0;
        let mut den = 1;

        while i < chars.len() {
            // sign
            let mut sign = 1;
            if chars[i] == '+' {
                i += 1;
            } else if chars[i] == '-' {
                sign = -1;
                i += 1;
            }

            // numerator
            let mut n = 0;
            while i < chars.len() && chars[i].is_digit(10) {
                n = n * 10 + (chars[i] as i32 - '0' as i32);
                i += 1;
            }

            i += 1; // skip '/'

            // denominator
            let mut d = 0;
            while i < chars.len() && chars[i].is_digit(10) {
                d = d * 10 + (chars[i] as i32 - '0' as i32);
                i += 1;
            }

            n *= sign;

            // combine fractions
            num = num * d + n * den;
            den = den * d;

            // simplify
            let g = gcd(num, den);
            num /= g;
            den /= g;
        }

        format!("{}/{}", num, den)
    }
}
