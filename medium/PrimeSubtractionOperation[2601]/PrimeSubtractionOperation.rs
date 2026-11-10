impl Solution {
    pub fn prime_sub_operation(nums: Vec<i32>) -> bool {
        let max = *nums.iter().max().unwrap() as usize;

        // Sieve of Eratosthenes
        let mut is_prime = vec![true; max + 1];
        is_prime[0] = false;
        is_prime[1] = false;

        for i in 2..=((max as f64).sqrt() as usize) {
            if is_prime[i] {
                for j in (i * i..=max).step_by(i) {
                    is_prime[j] = false;
                }
            }
        }

        let primes: Vec<i32> = (2..=max as i32).filter(|&p| is_prime[p as usize]).collect();

        let mut prev = 0;

        for x in nums {
            let limit = x - prev;

            let idx = primes.partition_point(|&p| p < limit);

            let current = if idx > 0 { x - primes[idx - 1] } else { x };

            if current <= prev {
                return false;
            }

            prev = current;
        }

        true
    }
}
