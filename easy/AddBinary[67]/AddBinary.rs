impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a;
        let mut b = b;
        let mut res = Vec::new();
        let mut carry = 0;
        while !a.is_empty() || !b.is_empty() {
            let x = a.pop().map_or(0, |x| (x == '1') as u8);
            let y = b.pop().map_or(0, |x| (x == '1') as u8);

            let val = x ^ y ^ carry;
            carry = (x & y) | (carry & (x ^ y));
            res.push(val);
        }
        if carry == 1 {
            res.push(1);
        }
        res.iter()
            .rev()
            .map(|&bit| if bit == 1 { '1' } else { '0' })
            .collect()
    }
}
