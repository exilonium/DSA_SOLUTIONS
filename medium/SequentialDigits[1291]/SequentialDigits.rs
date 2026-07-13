impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = Vec::new();

        for len in 2..=9 {
            for start in 1..=(10 - len) {
                let mut num = 0;

                for d in start..start + len {
                    num = num * 10 + d;
                }

                if num >= low && num <= high {
                    ans.push(num);
                }
            }
        }

        ans
    }
}
