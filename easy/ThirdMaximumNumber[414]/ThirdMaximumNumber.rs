impl Solution {
    pub fn third_max(nums: Vec<i32>) -> i32 {
        let mut max1 = i64::MIN;
        let mut max2 = i64::MIN;
        let mut max3 = i64::MIN;

        for num in nums {
            let num = num as i64;

            // Ignore duplicates
            if num == max1 || num == max2 || num == max3 {
                continue;
            }

            if num > max1 {
                max3 = max2;
                max2 = max1;
                max1 = num;
            } else if num > max2 {
                max3 = max2;
                max2 = num;
            } else if num > max3 {
                max3 = num;
            }
        }

        if max3 == i64::MIN {
            max1 as i32
        } else {
            max3 as i32
        }
    }
}
