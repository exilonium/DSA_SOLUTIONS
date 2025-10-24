// this is called boyer-moore majority vote element

impl Solution{
 
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut counter:i32=0;
        let mut max:i32=0;
        for i in nums.iter(){
            if counter == 0{
                max = *i;
                counter+=1;
            }
            else if max == *i{
                counter+=1;
            }
            else{
                counter-=1
            }
        }
        return max
    }

}
