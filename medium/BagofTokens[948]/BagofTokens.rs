impl Solution {
    pub fn bag_of_tokens_score(tokens: Vec<i32>, power: i32) -> i32 {
        let mut tokens = tokens;
        let mut power = power;
        tokens.sort_unstable();

        let (mut left, mut right) = (0usize, tokens.len());
        let mut score = 0;
        let mut max_score = 0;

        while left < right {
            if power >= tokens[left] {
                // Spend power to gain score
                power -= tokens[left];
                left += 1;
                score += 1;
                max_score = max_score.max(score);
            } else if score > 0 {
                // Spend score to regain power
                right -= 1;
                power += tokens[right];
                score -= 1;
            } else {
                break;
            }
        }

        max_score
    }
}
