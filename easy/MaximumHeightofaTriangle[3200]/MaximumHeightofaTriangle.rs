impl Solution {
    pub fn max_height_of_triangle(red: i32, blue: i32) -> i32 {
        fn height(mut a: i32, mut b: i32) -> i32 {
            let mut h = 0;
            let mut row = 1;

            while a >= row || b >= row {
                if h % 2 == 0 {
                    if a < row {
                        break;
                    }
                    a -= row;
                } else {
                    if b < row {
                        break;
                    }
                    b -= row;
                }

                h += 1;
                row += 1;
            }

            h
        }

        height(red, blue).max(height(blue, red))
    }
}
