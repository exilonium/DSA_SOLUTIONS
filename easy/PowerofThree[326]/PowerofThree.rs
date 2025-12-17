impl Solution {
    pub fn is_power_of_three(n: i32) -> bool {
        return n > 0 && 1162261467 % n == 0;
        // 116226147 this is 3^19 (3_i32.pow(19))
        // this is a mathematical property
    }
}
