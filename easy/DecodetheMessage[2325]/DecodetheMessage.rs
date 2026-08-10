impl Solution {
    pub fn decode_message(key: String, message: String) -> String {
        let mut map = [0u8; 26];
        let mut next = b'a';

        // Build the substitution table
        for c in key.bytes() {
            if c == b' ' {
                continue;
            }

            let idx = (c - b'a') as usize;

            if map[idx] == 0 {
                map[idx] = next;
                next += 1;
            }
        }

        // Decode the message
        message
            .bytes()
            .map(|c| {
                if c == b' ' {
                    b' '
                } else {
                    map[(c - b'a') as usize]
                }
            })
            .map(|c| c as char)
            .collect()
    }
}
