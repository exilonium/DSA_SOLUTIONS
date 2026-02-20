// 86 ms
use std::collections::HashMap;
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        let mut memo = HashMap::new();
        Self::solve(amount, &coins, &mut memo)
    }
    fn solve(rem: i32, coins: &[i32], memo: &mut HashMap<i32, i32>) -> i32 {
        if rem < 0 {
            return -1;
        }
        if rem == 0 {
            return 0;
        }
        if memo.contains_key(&rem) {
            return memo[&rem];
        }
        let mut min_count = i32::MAX;

        for coin in coins {
            let res = Self::solve(rem - coin, coins, memo);
            if res != -1 {
                min_count = min_count.min(1 + res);
            }
        }
        let res = if min_count == i32::MAX { -1 } else { min_count };

        memo.insert(rem, res);
        return memo[&rem];
    }
}
