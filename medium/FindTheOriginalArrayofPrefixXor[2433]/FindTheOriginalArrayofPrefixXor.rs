impl Solution {
    pub fn find_array(pref: Vec<i32>) -> Vec<i32> {
        let mut arr = vec![0; pref.len()];
        let mut prev = 0;
        for i in 0..pref.len() {
            arr[i] = pref[i] ^ prev;
            prev = pref[i];
        }
        arr
    }
}
