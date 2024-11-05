impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        
        // LIS 길이를 구하는 helper function. dp를 통해서 각 0..해당 원소까지의 lis 길이를 담음.
        fn calculate_lis(nums: &[i32]) -> Vec<i32> {
            let mut lis = Vec::new();
            let mut dp = vec![0; nums.len()];
    
            for i in 0..nums.len() {
                let pos = lis.binary_search(&nums[i]).unwrap_or_else(|x| x);
                if pos < lis.len() {
                    lis[pos] = nums[i];
                } else {
                    lis.push(nums[i]);
                }
                dp[i] = (pos + 1) as i32;
            }
            dp
        }
    
        // lis 길이를 뒤집어서 구하면 lds
        let left_lis = calculate_lis(&nums);
        let mut nums_reversed = nums.clone();
        nums_reversed.reverse();
        let right_lis_rev = calculate_lis(&nums_reversed);
        let right_lis: Vec<i32> = right_lis_rev.into_iter().rev().collect();
    
        // Find the maximum mountain length
        let mut max_mountain_len = 0;
        for k in 1..n - 1 {
            if left_lis[k] > 1 && right_lis[k] > 1 {
                max_mountain_len = max_mountain_len.max(left_lis[k] + right_lis[k] - 1);
            }
        }
    
        // Minimum removals to get the mountain array
        (n as i32) - max_mountain_len
    }
}