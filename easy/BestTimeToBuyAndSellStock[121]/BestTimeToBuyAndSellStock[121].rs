// optimal solution
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut maxprofit = i32::MIN;
        let mut lowestprice =i32::MAX;        
        for i in 0..prices.len(){
            lowestprice=std::cmp::min(lowestprice,prices[i]);
            maxprofit = std::cmp::max(maxprofit,prices[i]-lowestprice);
        }
        return maxprofit
    }
}
