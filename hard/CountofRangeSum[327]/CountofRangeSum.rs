impl Solution {
    pub fn count_range_sum(nums: Vec<i32>, lower: i32, upper: i32) -> i32 {
        fn merge_sort(
            sums: &mut Vec<i64>,
            temp: &mut Vec<i64>,
            left: usize,
            right: usize,
            lower: i64,
            upper: i64,
        ) -> i64 {
            if right - left <= 1 {
                return 0;
            }

            let mid = left + (right - left) / 2;

            let mut count = merge_sort(sums, temp, left, mid, lower, upper)
                + merge_sort(sums, temp, mid, right, lower, upper);

            let mut j = mid;
            let mut k = mid;

            for i in left..mid {
                while j < right && sums[j] - sums[i] < lower {
                    j += 1;
                }

                while k < right && sums[k] - sums[i] <= upper {
                    k += 1;
                }

                count += (k - j) as i64;
            }

            // Merge both sorted halves
            let (mut i, mut j) = (left, mid);

            for p in left..right {
                if i < mid && (j >= right || sums[i] <= sums[j]) {
                    temp[p] = sums[i];
                    i += 1;
                } else {
                    temp[p] = sums[j];
                    j += 1;
                }
            }

            sums[left..right].copy_from_slice(&temp[left..right]);

            count
        }

        let mut sums = vec![0i64];

        for num in nums {
            sums.push(sums.last().unwrap() + num as i64);
        }

        let n = sums.len();
        let mut temp = vec![0i64; n];

        merge_sort(&mut sums, &mut temp, 0, n, lower as i64, upper as i64) as i32
    }
}
