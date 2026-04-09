impl Solution {
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        while b != 0 {
            let tmp = b;
            b = a % b;
            a = tmp;
        }
        a
    }
    pub fn can_measure_water(x: i32, y: i32, target: i32) -> bool {
        x + y >= target && (x + y == target || target % Self::gcd(x, y))
    }
}
