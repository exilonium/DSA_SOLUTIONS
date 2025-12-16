// time limit exceeded 47/48 testcases passed
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut res = vec![];
        for i in 0..temperatures.len() {
            let mut counter = 0;
            let mut flag = 0;
            for j in i..temperatures.len() {
                if temperatures[i] >= temperatures[j] {
                    counter += 1;
                } else {
                    flag = 1;
                    break;
                }
            }
            if flag == 1 {
                res.push(counter);
                flag = 0;
            } else {
                res.push(0);
            }
        }
        res
    }
}
