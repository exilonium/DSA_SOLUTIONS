impl Solution {
    pub fn minimum_cost(cost1: i32, cost2: i32, cost_both: i32, need1: i32, need2: i32) -> i64 {
        let (c1, c2, cb, n1, n2) = (
            cost1 as i64,
            cost2 as i64,
            cost_both as i64,
            need1 as i64,
            need2 as i64,
        );

        if cb >= c1 + c2 {
            return n1 * c1 + n2 * c2;
        }

        let both = n1.min(n2);

        both * cb + (n1 - both) * c1.min(cb) + (n2 - both) * c2.min(cb)
    }
}
