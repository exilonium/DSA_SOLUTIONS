impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let weeks :i32 = n/7;
        let days :i32= n%7;
        let mut res = weeks*28 + 7*weeks*(weeks-1) /2; // add all the week data
        res += days*(weeks+1) + days * (days-1)/2; // add remaining data

        res

    }
}
