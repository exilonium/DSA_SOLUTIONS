/**
 * Forward declaration of guess API.
 * @param  num   your guess
 * @return 	     -1 if num is higher than the picked number
 *			      1 if num is lower than the picked number
 *               otherwise return 0
 * unsafe fn guess(num: i32) -> i32 {}
 */

impl Solution {
    unsafe fn guessNumber(n: i32) -> i32 {
        let mut low = 0;
        let mut high = n;
        let mut mid = low + (high - low) / 2;
        let mut g = guess(mid);
        while g != 0 {
            match g {
                1 => low = mid + 1,
                -1 => high = mid - 1,
                _ => {}
            }
            mid = low + (high - low) / 2;
            g = guess(mid);
        }
        mid
    }
}
