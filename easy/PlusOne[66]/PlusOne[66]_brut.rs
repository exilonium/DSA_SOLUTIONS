// this is very worse brut and it wont submit due to integer overflow of i32
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut num:i64 = 0;
        let mut temp =1;
        for i in digits.iter().rev(){
            num += i*temp;
            temp*=10;
        }
        num+=1;
        let mut res:Vec<i32>= Vec::new();
        for i in (num.to_string()).chars(){
            res.push(i.to_digit(10).unwrap() as i32);
        }
        return res
    }
}
