// this is optimal 
impl Solution {
    pub fn rearrange_array(nums: Vec<i32>) -> Vec<i32> {
        let mut sol = nums.clone();
        let mut pos = 0;
        let mut neg=1;
        for elem in nums.into_iter(){
            if elem>0{
                sol[pos]=elem;
                pos+=2;
            } else{
                sol[neg]=elem;
                neg+=2;
            }
        }
        return sol
    }
}
