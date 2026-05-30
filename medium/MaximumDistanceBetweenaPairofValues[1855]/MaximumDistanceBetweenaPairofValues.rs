impl Solution {
    pub fn max_distance(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            while j + 1 < nums2.len() && nums1[i] <= nums2[j + 1] {
                j += 1;
            }
            i += 1;
            j += 1;
        }
        (j - i) as i32
    }
}
