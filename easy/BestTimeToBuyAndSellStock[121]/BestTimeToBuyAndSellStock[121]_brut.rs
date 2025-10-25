// this is a brute-force approach
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = i32::MIN;
        // let mut min = i32::MAX;
        let mut start = 0;
        let mut end=0;

        for (i,_) in prices.iter().enumerate(){
            for j in i..prices.len(){
                let diff=prices[j]-prices[i];
                if diff>max{
                    max = diff;
                    start =i;
                    end =j;
                }
            }
            //println!("{}  {}",start,end);

        }
        return max
    }
}
