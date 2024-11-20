use std::collections::HashSet;

impl Solution {
    pub fn maximum_subarray_sum(nums: Vec<i32>, k: i32) -> i64 {
        let k = k as usize;
        let mut set = HashSet::with_capacity(k);
        let mut sum = 0i64;
        let mut max = 0i64;
        let mut start = 0usize;

        for end in 0..nums.len() {
            while set.contains(&nums[end]) || set.len() == k {
                set.remove(&nums[start]);
                sum -= nums[start] as i64;
                start += 1;
            }
            set.insert(&nums[end]);
            sum += nums[end] as i64;

            if set.len() == k {
                max = max.max(sum);
            }
        }
        max
    }
}