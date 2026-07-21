impl Solution {
    pub fn smallest_subsequence(s: String) -> String {
        let bytes = s.as_bytes();

        let mut last = [0usize; 26];
        for (i, &ch) in bytes.iter().enumerate() {
            last[(ch - b'a') as usize] = i;
        }

        let mut visited = [false; 26];
        let mut stack: Vec<u8> = Vec::new();

        for (i, &ch) in bytes.iter().enumerate() {
            let idx = (ch - b'a') as usize;

            if visited[idx] {
                continue;
            }

            while let Some(&top) = stack.last() {
                let top_idx = (top - b'a') as usize;

                if top > ch && last[top_idx] > i {
                    stack.pop();
                    visited[top_idx] = false;
                } else {
                    break;
                }
            }

            stack.push(ch);
            visited[idx] = true;
        }

        stack.into_iter().map(char::from).collect()
    }
}
