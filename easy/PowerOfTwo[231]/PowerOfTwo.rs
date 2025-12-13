impl Solution {
    pub fn is_power_of_two(n: i32) -> bool {
        n > 0 && (n & (n - 1)) == 0
        // checks if n is greater than zero
        // then bitwise and with the previous number
        // power of two are of bits 100000....000
        // subtracting one gives bits 0111111....111
        // doing and sets all bits to zero 00000000....000
    }
}
