impl Solution {
    pub fn flip_and_invert_image(image: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![];
        for row in image {
            let mut l = row.len();
            let mut tmp = vec![0; l];
            for i in 0..l {
                tmp[l - i - 1] = row[i] ^ 1; // xor to flip 0 and 1 and fliping horizontally using array;
            }
            res.push(tmp);
        }
        res
    }
}
