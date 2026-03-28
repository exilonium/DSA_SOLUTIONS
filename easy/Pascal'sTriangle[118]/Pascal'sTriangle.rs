//[1]
//[1,1]
//[1,2,1]
//[1,3,3,1]
//[1,4,6,4,1]
//[1,5,10,10,5,1]

impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = vec![];
        if num_rows == 0 {
            return res;
        }
        res.push(vec![1]);

        for i in 1..num_rows {
            let prev = res[(i - 1) as usize].clone(); // last element
            let mut curr = vec![1];

            for j in 1..i {
                curr.push(prev[(j - 1) as usize] + prev[j as usize])
            }

            curr.push(1);
            res.push(curr);
        }
        res
    }
}
