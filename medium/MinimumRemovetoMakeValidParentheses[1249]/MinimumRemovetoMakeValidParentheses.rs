impl Solution {
    pub fn min_remove_to_make_valid(s: String) -> String {
        let mut stack = Vec::new();
        let mut open = 0;

        for c in s.chars() {
            match c {
                '(' => {
                    open += 1;
                    stack.push(c);
                }
                ')' => {
                    if open > 0 {
                        open -= 1;
                        stack.push(c);
                    }
                }
                _ => stack.push(c),
            }
        }

        let mut ans = Vec::new();

        for c in stack.into_iter().rev() {
            match c {
                '(' if open > 0 => open -= 1,
                _ => ans.push(c),
            }
        }

        ans.reverse();
        ans.into_iter().collect()
    }
}
