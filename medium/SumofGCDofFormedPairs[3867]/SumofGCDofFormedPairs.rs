impl Solution {
    fn gcd(a: i64, b: i64) -> i64 {
        let mut a = a;
        let mut b = b;
        while b != 0 {
            let temp = b;
            b = a % b;
            a = temp;
        }
        a
    }

    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let mut max_i = 1_i64;
        let mut pref = vec![1; nums.len()];

        for (idx, num) in nums.iter().enumerate() {
            max_i = max_i.max(*num as i64);
            pref[idx] = Self::gcd(*num as i64, max_i);
        }

        pref.sort_unstable();
        let mut i = 0;
        let mut j = pref.len() - 1;
        let mut res = 0_i64;
        while i < j {
            res += Self::gcd(pref[i], pref[j]);
            i += 1;
            j -= 1;
        }
        res
    }
}
