// optimal dp Solution
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        if amount < 0 {
            return -1;
        }
        let amount = amount as usize;
        let mut dp = vec![amount as i32 + 1; amount + 1];
        dp[0] = 0;

        for i in 1..=amount {
            for &coin in &coins {
                let coin = coin as usize;
                if i >= coin {
                    dp[i] = dp[i].min(dp[i - coin] + 1);
                }
            }
        }
        if dp[amount] != amount as i32 + 1 {
            dp[amount]
        } else {
            -1
        }
    }
}
