use std::{collections::HashSet, vec};

impl Solution {
    pub fn remove(nums: &mut Vec<i32>, index: usize, k: usize) -> usize {
        if index < 0 || index >= nums.len() {
            return k;
        }

        for i in index..nums.len() - 1 {
            nums[i] = nums[i + 1];
        }

        k - 1
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        // value + 100 -> index
        let mut set = vec![false; 201];
        let mut k = nums.len();
        let mut index = 0;

        while index < k {
            let value = nums[index] as usize;
            if set[value + 100] { 
                k = Self::remove(nums, index, k);
            } else {
                set[value + 100] = true;
                index += 1;
            }
        }
        
        k as i32
    }
}