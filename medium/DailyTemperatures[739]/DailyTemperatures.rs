// unfortunately i wasnt able to reach the Solution all by myself
// this is the optimal code
impl Solution {
    pub fn daily_temperatures(temperatures: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<usize> = vec![];
        let mut res = vec![0; temperatures.len()];

        for i in 0..temperatures.len() {
            while let Some(&li) = stack.last() {
                if temperatures[li] < temperatures[i] {
                    stack.pop();
                    res[li] = (i - li) as i32;
                } else {
                    break;
                }
            }
            stack.push(i)
        }

        res
    }
}

