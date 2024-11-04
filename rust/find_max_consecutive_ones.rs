impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut max_len = 0;
        let mut len = 0;

        for n in nums {
            len = n * len + n;
            max_len = max_len.max(len);
        }
        max_len
    }
}