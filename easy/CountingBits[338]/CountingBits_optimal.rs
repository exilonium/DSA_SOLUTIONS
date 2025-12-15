impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let mut res = vec![0; (n + 1) as usize];
        for i in 0..=n {
            res[i as usize] = res[i as usize / 2] + (i & 1); // i&1 is same as i%2 basically odd num has last bit one so we adding that
            //res[i] = res[i >> 1] + (i & 1) as i32; this is same as above but more idiomatic :)
            //but we have to define i as usize
        }
        res
    }
}
