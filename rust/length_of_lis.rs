impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut last = stack.last();
        
        for n in nums {
            if last == None || *last.unwrap() < n {
                stack.push(n);
            } else {
                let idx = stack.binary_search(&n).unwrap_or_else(|x| x);
                stack[idx] = n;
            }
            last = stack.last();
        }
        stack.len() as i32
    }    
}