// solution by xkatianx fahh this is so fkin cool O(1)
impl Solution {
    pub fn last_remaining(n: i32) -> i32 {
        let mut t = n | n >> 1;
        t = t | t >> 2;
        t = t | t >> 4;
        t = t | t >> 8;
        t = t | t >> 16;
        (t + 1 >> 1) - (t & !n & 0x3aaaaaaa)
    }
}
