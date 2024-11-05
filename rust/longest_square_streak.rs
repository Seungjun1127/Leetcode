use std::collections::HashSet;

impl Solution {
    pub fn longest_square_streak(nums: Vec<i32>) -> i32 {
		let mut set = HashSet::new();

        for i in 0..nums.len() {
            set.insert(nums[i]);
        }

        let mut ans = -1;
        for i in 0..nums.len() {
            let mut count = 0;
            let mut sq = nums[i] as i128;
            loop {
                if sq <= 100000 && set.contains(&(sq as i32)) {
                    count += 1;
                    sq = sq * sq;
                    if sq > 100000 {
                        break
                    }

                } else {
                    break
                }
            }
            if count > 1 && count > ans {
                ans = count;
            }
        }
        ans
    }
}
