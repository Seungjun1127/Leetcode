use std::collections::HashMap;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut max_len = 0;
        let mut start = 0;
        let mut map = HashMap::with_capacity(1024);

        for (end, ch) in s.chars().enumerate() {
            if let Some(&dupl) = map.get(&ch) {
                start = start.max(dupl + 1);
            }

            map.insert(ch, end);
            max_len = max_len.max(end - start + 1);
        }
        max_len as i32
    }
}