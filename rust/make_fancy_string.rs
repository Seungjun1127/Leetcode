impl Solution {
    pub fn make_fancy_string(s: String) -> String {
        let mut ans = String::with_capacity(s.len());
        let mut one_before = 'B';
        let mut two_before = 'A';

        for ch in s.chars().into_iter() {
            if ch == one_before && ch == two_before {
                continue;
            }
            two_before = one_before;
            one_before = ch;
            ans.push(ch);
        }
        ans
    }
}