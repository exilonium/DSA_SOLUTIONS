use std::collections::HashMap;

impl Solution {
    pub fn interchangeable_rectangles(rectangles: Vec<Vec<i32>>) -> i64 {
        let mut freq: HashMap<(i32, i32), i64> = HashMap::new();
        let mut ans = 0i64;

        for rect in rectangles {
            let w = rect[0];
            let h = rect[1];

            // Reduce the width-to-height ratio
            let g = Self::gcd(w, h);
            let ratio = (w / g, h / g);

            let count = freq.entry(ratio).or_insert(0);
            ans += *count;
            *count += 1;
        }

        ans
    }

    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let r = a % b;
            a = b;
            b = r;
        }
        a
    }
}
