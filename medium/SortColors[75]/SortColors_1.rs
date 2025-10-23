impl Solution {
    pub fn sort_colors(nums: &mut Vec<i32>) {
        let mut zero = 0;
        let mut one = 0;
        let mut two = 0;

        for val in nums.iter() {
            match *val {
                0 => zero += 1,
                1 => one += 1,
                2 => two += 1,
                _ => {} // handle or ignore other values
            }
        }

        //println!("{} {} {}", zero, one, two);
        let mut mapit = 0;
        for i in 0..nums.len(){
            if zero >0 {
                nums[i]=0;
                zero-=1;
            }
            else if one >0{
                nums[i]=1;
                one-=1;
            }
            else if two >0{
                nums[i]=2;
                two-=1;
            }
        }
    }
}
// one of the shittiest solution but it works
// its constan space tho ive to make it work somehow in onepass
