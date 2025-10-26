// this is brut 
impl Solution {
    pub fn rearrange_array(num: Vec<i32>) -> Vec<i32> {
        let mut nums = num.clone();
        let mut pos = Vec::new();
        let mut neg = Vec::new();
        for &i in nums.iter(){
            if i>=0{
                pos.push(i);
            }else{
                neg.push(i);
            }
        }
        for i in 0..nums.len(){
            if i%2==0{
                nums[i]=pos[i/2];
            }else{
                nums[i]=neg[i/2];
            }
        }
        return nums.to_vec()
    }
}
