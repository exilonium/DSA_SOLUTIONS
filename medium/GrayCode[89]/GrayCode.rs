impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut res = vec![];
        let total_num = 1 << n;
        // say n = 2  total num = 4(100 in binary)
        // say n = 10 total_num = 1024 (10000000000)
        // basically we are raising it by two.
        for i in 0..total_num {
            let gray = i ^ (i >> 1);
            res.push(gray);
        }
        res
    }
}
