// this kinda better with 0ms time but bad memory
impl Solution {
    pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
        let mut flag=true;
        let mut last = digits.len()-1;
        let mut res:Vec<i32>=digits.clone();
        if res == vec![9]{
            return vec![1,0];
        }
        while flag {
            if res[last]==9{
                res[last]=0;
                if last == 0{
                    res.insert(0, 1);
                    return res
                }
                last-=1;
                
            } else {
                res[last]+=1;
                flag = false;
            }
        }
        res

    }
}
