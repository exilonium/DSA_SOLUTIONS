const HEX_MAP: [char; 16] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
];
impl Solution {
    pub fn to_hex(num: i32) -> String {
        if num == 0 {
            return "0".to_string();
        }
        let mut res = Vec::with_capacity(8);
        let mut n = num as u32; // converting it to unsigned
        while n != 0 {
            let idx = (n & 0b1111) as usize; // this basically doing bitwise AND with 15 (15.to_hex() = 0b1111)
            n >>= 4; // shifting last 4 bits cause they are done working with
            res.push(HEX_MAP[idx]); // pushing the corresponding hex value to the res
        }
        res.into_iter().rev().collect::<String>()
        // String::from_iter(res.into_iter().rev())
        // both are correct
    }
}
