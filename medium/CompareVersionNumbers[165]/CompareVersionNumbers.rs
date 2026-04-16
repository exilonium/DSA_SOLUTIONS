impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        let v1: Vec<i32> = version1
            .split(".")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        let v2: Vec<i32> = version2
            .split(".")
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        let size1 = v1.len();
        let size2 = v2.len();

        for idx in 0..size1.max(size2) {
            if idx > size1 - 1 && v2[idx] > 0 {
                return -1;
            }
            if idx > size2 - 1 && v1[idx] > 0 {
                return 1;
            }
            if idx > size1 - 1 || idx > size2 - 1 {
                continue;
            }
            if v1[idx] < v2[idx] {
                return -1;
            } else if v1[idx] > v2[idx] {
                return 1;
            }
        }
        0
    }
}
