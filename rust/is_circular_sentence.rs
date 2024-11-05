impl Solution {
    pub fn is_circular_sentence(sentence: String) -> bool {
        let split: Vec<&str> = sentence.split(' ').collect();

        let last_index = split.len() - 1;
        let next = |i: usize| {
            if i == last_index {
                0
            } else {
                i + 1
            }
        };

        let is_circular = |s1: &str, s2: &str| -> bool {
            s1.chars().last() == s2.chars().next()
        };

        for i in 0..split.len() {
            let curr_word = split[i];
            let next_word = split[next(i)];

            if !is_circular(curr_word, next_word) {
                return false;
            }
        }
        
        true
    }
}