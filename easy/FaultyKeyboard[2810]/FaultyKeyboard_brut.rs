// brut
impl Solution {
    pub fn final_string(s: String) -> String {
        let mut res = String::with_capacity(s.len());
        for i in s.chars() {
            match i {
                'i' => {
                    let mut bytes = res.into_bytes();
                    bytes.reverse();
                    res = String::from_utf8(bytes).expect("valid utf-8");
                }
                _ => res.push(i),
            }
        }
        res
    }
}
