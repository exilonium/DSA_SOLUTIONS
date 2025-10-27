impl Solution {
    pub fn next_permutation(nums: &mut Vec<i32>)-> &mut Vec<i32>{
        let mut idx=usize::MAX;
        if nums.len()==1{
            return nums;
        }
        for i in (0..=nums.len()-2).rev(){
            if nums[i]<nums[i+1]{
                idx=i;
                break;
            }
        }
        if idx==usize::MAX{
            nums.reverse();
            return nums;
        }
        for i in (0..nums.len()).rev(){
            if nums[i]>nums[idx]{
                nums.swap(idx,i);
                break;
            }
        }
        nums[idx + 1..].reverse();
        nums

    }
}
