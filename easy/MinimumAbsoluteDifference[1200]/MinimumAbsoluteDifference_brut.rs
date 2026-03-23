// absolute brut TLE
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort();
        let mut min = i32::MAX;
        let mut size = arr.len();
        for i in 0..size {
            for j in i..size - 1 {
                let diff = arr[i].abs_diff(arr[j + 1]);
                min = min.min(diff as i32);
            }
        }
        let mut res = vec![];
        for i in 0..size {
            for j in i..size - 1 {
                let diff = arr[i] - arr[j + 1];
                if diff == -min {
                    res.push(vec![arr[i], arr[j + 1]]);
                } else if diff == min {
                    res.push(vec![arr[j + 1], arr[i]])
                }
            }
        }
        res
    }
}
