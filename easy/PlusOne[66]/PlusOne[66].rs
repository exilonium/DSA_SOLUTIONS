// optimal
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut digits = digits;
        for x in digits.iter_mut().rev(){
            match *x == 9{
                true => *x=0,
                false => {
                    *x+=1;
                    return digits
                }
            }
        }
        digits.insert(0,1);
        digits
    }
}
