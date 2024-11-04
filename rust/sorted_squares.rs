impl Solution {
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let mut sqs: Vec<i32> = nums.iter().map(|n| n * n).collect();
        sqs.sort();
        sqs
    }
}