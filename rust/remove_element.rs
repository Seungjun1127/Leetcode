impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut k = nums.len();
        let mut index = 0;

        while index < k {
            if nums[index] == val {
                for i in index..nums.len() - 1 {
                    nums[i] = nums[i + 1];
                }
                k -= 1;
            } else {
                index += 1;
            }
        }
        k as i32
    }
}