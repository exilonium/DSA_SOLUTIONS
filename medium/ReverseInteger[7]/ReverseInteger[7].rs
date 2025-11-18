impl Solution {
    pub fn reverse(x: i32) -> i32 {
        let mut y = x;
        let mut temp:i32 =0;
        let mut rev:i64 =0;
        while y!=0{
            temp = y%10;
            rev = rev*10+(temp as i64);
            y = y/10;
        }
        if rev>(i32::MAX as i64)|| rev<(i32::MIN as i64){
            return 0
        }
        rev as i32
    }
}
