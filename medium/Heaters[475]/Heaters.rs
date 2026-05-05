impl Solution {
    pub fn find_radius(houses: Vec<i32>, heaters: Vec<i32>) -> i32 {
        fn closest_values(arr1: Vec<i32>, mut arr2: Vec<i32>) -> i32 {
            assert!(!arr2.is_empty(), "arr2 must not be empty");

            let mut max_dist = 0;
            // Sort the searchable array
            arr2.sort_unstable();

            for x in arr1 {
                let closest = match arr2.binary_search(&x) {
                    Ok(i) => arr2[i],
                    Err(i) => {
                        if i == 0 {
                            arr2[0]
                        } else if i == arr2.len() {
                            arr2[arr2.len() - 1]
                        } else {
                            let left = arr2[i - 1];
                            let right = arr2[i];
                            if (x - left).abs() <= (x - right).abs() {
                                left
                            } else {
                                right
                            }
                        }
                    }
                };
                let temp = (x - closest).abs();
                if temp > max_dist {
                    max_dist = temp;
                }
            }

            max_dist
        }

        let temp = closest_values(houses, heaters);

        return temp;
    }
}
