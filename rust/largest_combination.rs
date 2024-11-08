impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut max_size = 0;
        for i in 0..32 {
            let mut count = 0;
            for n in &candidates {
                if (n >> i) & 1 == 1 {
                    count += 1;
                }
            }
            if count > max_size {
                max_size = count;
            }
        }
        max_size
    }
}