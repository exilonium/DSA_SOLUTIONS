impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut res= 0;
        for i in s.chars().rev(){
            if i!=' '{res+=1;}
            else if res>0{
                return res;
            }
        }
        res
    }
}
