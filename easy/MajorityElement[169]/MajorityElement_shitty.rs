// this was another shitty solution
impl Solution {
    
    fn merge(left:&Vec<i32>, right:&Vec<i32>) -> Vec<i32>{
        let mut i=0;
        let mut j=0;
        let mut merged:Vec<i32>= Vec::new();
        while i< left.len() && j <right.len(){
            if left[i]<right[j]{
                merged.push(left[i]);  
                i+=1; 
            }else{
                merged.push(right[j]);
                j+=1;
            }
        }
        if i<left.len(){
            while i<left.len(){
                merged.push(left[i]);
                i+=1;
            }
        }
        if j<right.len(){
            while j< right.len(){
                merged.push(right[j]);
                j+=1;
            }
        }
        merged
    }
    fn merge_sort(vec:&Vec<i32>)-> Vec<i32>{
        if vec.len()<2{
            return vec.to_vec();
        }
        else{
            let mid = vec.len()/2;

            let left = Self::merge_sort(&vec[0..mid].to_vec());
            let right = Self::merge_sort(&vec[mid..].to_vec());
            let merged = Self::merge(&left,&right);
            merged
        }
    }
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let soln = Self::merge_sort(&nums);
        println!("{:?}",soln);
        let mut count = 0;
        let n = soln.len();
        for i in 0..soln.len(){
            if soln[i]==soln[n/2]{
                return soln[n/2]

            }
        }
        return 0
    }

}
