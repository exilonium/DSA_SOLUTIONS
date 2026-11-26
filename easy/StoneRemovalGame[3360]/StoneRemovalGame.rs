impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut take = 10;

        while n >= take {
            n -= take;
            take -= 1;
        }

        take % 2 == 1
    }
}
