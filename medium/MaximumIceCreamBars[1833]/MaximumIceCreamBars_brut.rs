impl Solution {
    pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
        let mut costs = costs;
        costs.sort_unstable();
        let mut count = 0;
        let mut coins = coins;
        for i in costs {
            if i <= coins {
                count += 1;
                coins -= i;
            } else {
                return count;
            }
        }
        count
    }
}
