// we can also use the hashtable
impl Solution {
    pub fn max_number_of_balloons(text: String) -> i32 {
        let mut wordmap = vec![0; 5];
        for c in text.chars() {
            match c {
                'b' => wordmap[0] += 2,
                'a' => wordmap[1] += 2,
                'l' => wordmap[2] += 1,
                'o' => wordmap[3] += 1,
                'n' => wordmap[4] += 2,
                _ => (),
            }
        }
        let mut res = i32::MAX;
        for i in wordmap {
            res = res.min(i);
        }
        res / 2
    }
}
