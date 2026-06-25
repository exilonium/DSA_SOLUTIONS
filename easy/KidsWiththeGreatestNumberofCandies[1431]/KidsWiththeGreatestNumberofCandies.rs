impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let size = candies.len();
        let mut res = vec![false; size];

        let mut maxx = 0;
        for i in &candies {
            maxx = maxx.max(*i);
        }

        for i in 0..size {
            if candies[i] + extra_candies >= maxx {
                res[i] = true;
            }
        }

        res
    }
}
