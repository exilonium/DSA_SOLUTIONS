// optimal solution using log(n) solution

impl Solution {
    pub fn peak_index_in_mountain_array(arr: Vec<i32>) -> i32 {
        let mut high = arr.len()-1;
        let mut low = 0;
        while low<high{
            let mut mid = low + (high-low)/2;
            if arr[mid+1]> arr[mid]{
                low= mid +1;
            } else{
                high = mid;
            }
        }
        low as _ // same as low as i32 it just defaults to the return type
    }
}
