impl Solution {
    pub fn get_maximum_xor(nums: Vec<i32>, maximum_bit: i32) -> Vec<i32> {
        let mut cumul_xor = Vec::new();
        let mut register = 0;
        let mask = (1 << maximum_bit) - 1;

        for &n in &nums {
            register ^= n;
            cumul_xor.push(register.clone());
        }
        cumul_xor.into_iter().rev().map(|x| x ^ mask).collect()
    }
}