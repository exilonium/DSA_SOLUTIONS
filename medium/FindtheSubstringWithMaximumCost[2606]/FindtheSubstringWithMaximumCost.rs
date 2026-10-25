impl Solution {
    pub fn maximum_cost_substring(s: String, chars: String, vals: Vec<i32>) -> i32 {
        let mut cost = [0; 26];

        for i in 0..26 {
            cost[i] = (i as i32) + 1;
        }

        for (c, v) in chars.bytes().zip(vals.iter()) {
            cost[(c - b'a') as usize] = *v;
        }

        let mut current = 0;
        let mut answer = 0;

        for c in s.bytes() {
            let value = cost[(c - b'a') as usize] as i32;

            current = (current + value).max(0);
            answer = answer.max(current);
        }

        answer
    }
}
