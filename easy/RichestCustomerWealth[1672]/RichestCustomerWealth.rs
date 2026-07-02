impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut biggest_money = 0;
        for banks in accounts {
            let value = banks.into_iter().sum();
            biggest_money = biggest_money.max(value);
        }
        biggest_money
    }
}
