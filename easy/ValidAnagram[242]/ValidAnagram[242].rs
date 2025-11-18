// kinda brute but have 1 ms time
use std::collections::HashMap;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len()!=t.len(){
            return false
        }
        let mut map: HashMap<char,i32>= HashMap::new();
        for i in s.chars(){
            map.entry(i).and_modify(|counter| *counter += 1).or_insert(1);
        }
        //println!("{map:?}");
        for i in t.chars(){
            map.entry(i).and_modify(|counter| *counter -= 1);

        }
        //println!("{map:?}");
        for value in map.values(){
            if *value!=0{
                return false
            }
        }

    true
    }
}
