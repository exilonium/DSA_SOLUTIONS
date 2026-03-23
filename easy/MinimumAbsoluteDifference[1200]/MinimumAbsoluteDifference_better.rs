// fahh
impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut arr = arr;
        arr.sort_unstable();
        let mut min = i32::MAX;
        let mut size = arr.len();

        for i in 0..size - 1 {
            min = min.min(arr[i + 1] - arr[i]);
        }

        let mut res = vec![];

        for i in 0..size - 1 {
            let diff = arr[i + 1] - arr[i];
            if diff == min {
                res.push(vec![arr[i], arr[i + 1]]);
            }
        }
        res
    }
}
