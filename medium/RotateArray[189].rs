impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let mut k = k%nums.len() as i32;
        if nums.len()==1 || k==0{
            return() 
        }
        let mut left:usize = 0;
        let mut right:usize= nums.len()-1;
        while left<right{
            let temp = nums[left];
            nums[left]= nums[right];
            nums[right]= temp;
            
            right-=1;
            left+=1;
        }
        left = 0;
        right = nums.len()-1;
        let mut shift:usize = k as usize-1;
        while left<shift{
            let temp = nums[left];
            nums[left]= nums[shift];
            nums[shift]= temp;
            left+=1;
            shift-=1;
        }
        shift= k as usize;
        while shift<right{
            let temp = nums[shift];
            nums[shift]= nums[right];
            nums[right]= temp;
            shift +=1;
            right-=1;
        }
        
    }
}
