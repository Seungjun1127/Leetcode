impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        let mut sum = 0;
        let mut ans: Vec<i32> = Vec::new();

        for i in 0..nums.len() {
            sum += nums[i];
            ans.push(sum);
        }
        ans
    }
}