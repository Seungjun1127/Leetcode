use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_nums = HashMap::new();
        for (index, value) in nums.iter().enumerate() {
            hash_nums.insert(value, index);
        }

        for i in 0..nums.len() {
            let j = target - nums[i] as i32;
            if hash_nums.contains_key(&j) && i < hash_nums.get(&j).unwrap().to_owned() {
                return vec![i as i32, hash_nums.get(&j).unwrap().to_owned() as i32]
            }
        }
        vec![0, 0]
    }
}