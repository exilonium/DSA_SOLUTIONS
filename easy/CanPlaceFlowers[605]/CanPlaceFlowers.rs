impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut flowerbed = flowerbed; // to modify the flowerbed
        let mut count = 0;
        for i in 0..flowerbed.len() {
            if flowerbed[i] == 1 {
                continue;
            }

            if (i == 0 || flowerbed[i - 1] == 0)
                && (i == flowerbed.len() - 1 || flowerbed[i + 1] == 0)
            {
                flowerbed[i] = 1; // planting 
                count += 1; // increasing the planting count
            }
        }
        count >= n
    }
}
