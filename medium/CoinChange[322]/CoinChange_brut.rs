// tle in [1,2,3] and amount = 100
impl Solution {
    pub fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
        Self::helper(&coins, amount)
    }

    fn helper(coins: &[i32], amount: i32) -> i32 {
        if amount < 0 {
            return -1;
        }
        if amount == 0 {
            return 0;
        }

        let mut min_count = i32::MAX;

        for &coin in coins {
            let res = Self::helper(coins, amount - coin);
            if res != -1 {
                min_count = min_count.min(res + 1);
            }
        }

        if min_count == i32::MAX { -1 } else { min_count }
    }
}
