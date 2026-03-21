impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut freq_list = [0; 26];

        magazine
            .bytes()
            .for_each(|c| freq_list[c as usize - 97] += 1);
        ransom_note
            .bytes()
            .for_each(|c| freq_list[c as usize - 97] -= 1); // 97 is basically 'a' as u32

        freq_list.into_iter().all(|x| x >= 0) // all method checks if they all satisfy the given condition
    }
}
