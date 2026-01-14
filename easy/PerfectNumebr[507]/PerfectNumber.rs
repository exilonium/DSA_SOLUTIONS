impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        let mut counter = 1;
        let mut sum = 0;
        while counter <= num.isqrt() {
            if num % counter == 0 {
                sum += counter;
                if counter != num / counter {
                    sum += num / counter;
                }
            }
            counter += 1;
        }
        sum -= num; // removing extra num which got added when counter =1
        sum == num
    }
}
