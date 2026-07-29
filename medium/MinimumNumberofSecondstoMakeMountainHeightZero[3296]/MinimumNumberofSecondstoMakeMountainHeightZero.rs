impl Solution {
    pub fn min_number_of_seconds(mountain_height: i32, worker_times: Vec<i32>) -> i64 {
        let h = mountain_height as i64;

        let mut lo = 0_i64;
        let mut hi = worker_times.iter().min().unwrap().to_owned() as i64 * h * (h + 1) / 2;

        while lo < hi {
            let mid = (lo + hi) / 2;

            let mut removed = 0_i64;

            for &t in &worker_times {
                let t = t as i64;

                let mut l = 0_i64;
                let mut r = h;

                while l < r {
                    let m = (l + r + 1) / 2;
                    if t * m * (m + 1) / 2 <= mid {
                        l = m;
                    } else {
                        r = m - 1;
                    }
                }

                removed += l;
                if removed >= h {
                    break;
                }
            }

            if removed >= h {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }

        lo
    }
}
