impl Solution {
    pub fn min_changes(s: String) -> i32 {
        let mut s = s.clone();
        let mut count = 0;
        let mut left = s.pop();
        let mut right = s.pop();

        while left != None && right != None {
            if left != right {
                count += 1;
            }
            left = s.pop();
            right = s.pop();
        }
        count
    }
}