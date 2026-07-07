impl Solution {
    pub fn sum_and_multiply(mut n: i32) -> i64 {
        let mut x = 0;
        let mut sum = 0;
        let mut place = 1;

        while n > 0 {
            let d = n % 10;
            sum += d;

            if d != 0 {
                x += d * place;
                place *= 10;
            }

            n /= 10;
        }

        (x as i64) * (sum as i64) // doing it as ((x * sum)) as i64 will cause the overflow and
        // wrong answer
    }
}
