impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let digit = |num: i32| {
            num.ilog10() + 1
        };

        let mut ans = 0;
        for num in nums {
            if digit(num) % 2 == 0 {
                ans += 1;
            }
        }
        ans
    }
}