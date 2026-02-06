impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n <= 2 {
            return 0;
        }
        let mut count = 0;
        let n = n as usize;
        let mut isprime = vec![true; n];
        isprime[0] = false;
        isprime[1] = false;

        for i in 2..n {
            if isprime[i] {
                count += 1;
                let mut j = 2 * i;
                while (j < n) {
                    isprime[j] = false;
                    j += i;
                }
            }
        }
        count
    }
}
