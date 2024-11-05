pub struct Solution;

impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut cnt = 0;
        let mut n = num;

        while n > 0 {
            if n % 2 == 1 {
                n -= 1;
            } else {
                n /= 2;
            }
            cnt += 1;
        }
        cnt
    }
}