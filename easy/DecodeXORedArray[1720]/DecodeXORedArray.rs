impl Solution {
    pub fn decode(encoded: Vec<i32>, first: i32) -> Vec<i32> {
        let mut size = encoded.len() + 1;
        let mut res = vec![0; size];
        let mut prev = first;
        res[0] = first;
        for i in 1..size {
            prev = encoded[i - 1] ^ prev;
            res[i] = prev;
        }
        res
    }
}
