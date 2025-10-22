impl Solution {
    pub fn move_zeroes(nums: &mut Vec<i32>) {
        let mut zeroidx :usize= 0; 
        for k in 0..nums.len(){
            if nums[k]!=0{
                nums.swap(k,zeroidx);
                zeroidx+=1;
            }       
        }

        
    }
}
