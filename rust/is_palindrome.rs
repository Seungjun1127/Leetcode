impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x < 0 {
            return false;
        }
        if x == 0 {
            return true;
        }

        let digit = x.ilog10() + 1;

        for i in 1..=(digit / 2) {
            if (x / 10_i32.pow(digit-i)) % 10 != (x % 10_i32.pow(i)) / 10_i32.pow(i-1) {
                return false;
            }
        }
        true
    }
}