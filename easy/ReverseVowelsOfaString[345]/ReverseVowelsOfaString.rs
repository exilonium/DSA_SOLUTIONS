//optimal
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let mut inpstr: Vec<char> = s.chars().collect();
        if inpstr.len() <= 1 {
            return s;
        }
        let mut low: usize = 0;
        let mut high: usize = inpstr.len() - 1;

        let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];

        while low < high {
            while low < high && !vowels.contains(&inpstr[low]) {
                low += 1;
            }
            while low < high && !vowels.contains(&inpstr[high]) {
                high -= 1;
            }
            if low < high {
                inpstr.swap(high, low);
                low += 1;
                high -= 1;
            }
        }
        inpstr.iter().collect()
    }
}
