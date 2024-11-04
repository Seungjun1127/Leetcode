impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        let shift = |s: &str| -> String {
            if s.is_empty() {
                return s.to_string();
            }
            
            let first_char = &s[0..1];
            let rest = &s[1..];
            
            format!("{}{}", rest, first_char)
        };

        let mut s = s;
        for _ in 0..s.len() {
            if s == goal {
                return true;
            }
            s = shift(&s);
        }
        false
    }
}