// 0ms solution
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut positive = true;
        let mut res = 0;
        let mut step = 0;
        let mut digit = -1;
        for i in s.chars() {
            match i {
                '0' => {
                    digit = -1;
                    if res != 0 {
                        digit = 0;
                    }
                }
                '1' => digit = 1,
                '2' => digit = 2,
                '3' => digit = 3,
                '4' => digit = 4,
                '5' => digit = 5,
                '6' => digit = 6,
                '7' => digit = 7,
                '8' => digit = 8,
                '9' => digit = 9,
                '-' => {
                    if step != 0 {
                        break;
                    }
                    positive = false;
                    digit = -1
                }
                '+' => {
                    if step != 0 {
                        break;
                    }
                    positive = true;
                    digit = -1;
                }
                ' ' => {
                    if step >= 1 {
                        break;
                    }
                    digit = -1;
                    step -= 1;
                }
                _ => break,
            }
            if digit != -1 {
                if res > (i32::MAX - digit) / 10 {
                    if !positive {
                        return i32::MIN;
                    } else {
                        return i32::MAX;
                    }
                } else {
                    res = res * 10 + digit;
                }
            }
            step += 1;
        }
        if !positive {
            return -res;
        }
        res
    }
}
