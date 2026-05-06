use std::cmp::max;
impl Solution {
    fn solve(start: i32, end: i32, mem: &mut Vec<Vec<i32>>) -> i32 {
        if start >= end {
            return 0;
        }
        if mem[start as usize][end as usize] != -1 {
            return mem[start as usize][end as usize];
        }

        let mut big = i32::MAX;

        for i in start..end {
            big = big.min(i + max(Self::solve(start, i - 1, mem), Self::solve(i + 1, end, mem)));
        }
        mem[start as usize][end as usize] = big;
        big
    }
    pub fn get_money_amount(n: i32) -> i32 {
        let mut mem = vec![vec![-1; n as usize + 1]; n as usize + 1];
        Self::solve(1, n, &mut mem)
    }
}
