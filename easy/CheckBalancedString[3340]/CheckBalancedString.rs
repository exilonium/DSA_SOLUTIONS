impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let (mut s1, mut s2) = (0, 0);
        for (i, ch) in num.chars().enumerate() {
            let digit = ch.to_digit(10).unwrap() as i32;
            if i % 2 == 0 {
                s1 += num[i..i + 1].parse::<i32>().unwrap();
            } else {
                s2 += num[i..i + 1].parse::<i32>().unwrap();
            }
        }
        s1 == s2
    }
}
