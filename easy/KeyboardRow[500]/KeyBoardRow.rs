use std::collections::HashSet;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let a = String::from("qwertyuiop");
        let b = String::from("asdfghjkl");
        let c = String::from("zxcvbnm");
        let first: HashSet<char> = a.chars().collect();
        let sec: HashSet<char> = b.chars().collect();
        let third: HashSet<char> = c.chars().collect();
        let mut go = 0;
        let mut res = vec![];
        for i in words {
            go = 0;
            let mut scammer = false;
            for j in i.chars() {
                let j = j.to_ascii_lowercase();
                if first.contains(&j) {
                    go = 1;
                    break;
                }
                if sec.contains(&j) {
                    go = 2;
                    break;
                }
                if third.contains(&j) {
                    go = 3;
                    break;
                }
            }
            match go {
                1 => {
                    for k in i.chars() {
                        if !first.contains(&k.to_ascii_lowercase()) {
                            scammer = true;
                            break;
                        }
                    }
                }
                2 => {
                    for k in i.chars() {
                        if !sec.contains(&k.to_ascii_lowercase()) {
                            scammer = true;
                            break;
                        }
                    }
                }
                3 => {
                    for k in i.chars() {
                        if !third.contains(&k.to_ascii_lowercase()) {
                            scammer = true;
                            break;
                        }
                    }
                }
                _ => {}
            }
            if !scammer {
                res.push(i);
            }
        }
        res
    }
}
