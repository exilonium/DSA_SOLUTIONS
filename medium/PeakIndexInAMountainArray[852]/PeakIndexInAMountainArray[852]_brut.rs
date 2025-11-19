// brut o(n) complexity task is to solve it in log(n)

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        for i in 0..arr.len()-1{
            if arr[i]>arr[i+1]{
                return (i) as i32;
            }
        }
        return 0
    }
}
