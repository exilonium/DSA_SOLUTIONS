impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_vec: Vec<i32> = vec![0; 26];
        let mut t_vec: Vec<i32> = vec![0; 26];

        for i in s.bytes() {
            let ch = i - b'a';
            s_vec[ch as usize] += 1;
        }
        for i in t.bytes() {
            let ch = i - b'a';
            t_vec[ch as usize] += 1;
        }
        let mut res: i32 = 0;
        for i in 0..26 {
            res += (t_vec[i] - s_vec[i]).abs();
        }
        res / 2 // dividing by 2 as it contains all the moves so it counted them twice
    }
}
