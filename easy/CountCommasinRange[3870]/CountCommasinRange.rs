impl Solution {
    pub fn count_commas(n: i32) -> i32 {
        // let mut count = 0;
        // for i in 1..=n{
        //     if i > 999{
        //         count+=1;
        //     }
        // }
        // count

        0.max(n - 999)
    }
}
