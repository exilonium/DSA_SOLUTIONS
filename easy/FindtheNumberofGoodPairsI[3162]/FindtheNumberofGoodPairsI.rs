impl Solution {
    pub fn number_of_pairs(nums1: Vec<i32>, nums2: Vec<i32>, k: i32) -> i32 {
        let n = nums1.len();
        let m = nums2.len();
        let mut res = 0;

        for i in 0..n {
            let first = nums1[i];
            for j in 0..m {
                let sec = nums2[j];
                if first % (sec * k) == 0 {
                    res += 1;
                }
            }
        }
        res
    }
}
