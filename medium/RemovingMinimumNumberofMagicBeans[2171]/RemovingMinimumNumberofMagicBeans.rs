impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        beans.sort_unstable();

        let sum: i64 = beans.iter().map(|&x| x as i64).sum();
        let n = beans.len();

        let mut ans = sum;

        for i in 0..n {
            let remaining = (n - i) as i64 * beans[i] as i64;
            ans = ans.min(sum - remaining);
        }

        ans
    }
}
