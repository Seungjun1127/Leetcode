impl Solution {
    fn exchange_with(stack: &mut Vec<i32>, n: i32) {
        let mut target = 0;
        let mut upper = stack.len() - 1;
        let mut step_size = stack.len() / 2;

        while stack[target] < n {
            if stack[(target + upper) / 2] < n {
                target += step_size;
                if step_size == 0 {
                    target += 1;
                }
            } else {
                upper -= step_size;
                if step_size == 0 {
                    upper -= 1;
                }
            }

            step_size /= 2;
        }
        stack[target] = n;
    }
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        let mut stack = Vec::new();
        let mut last = stack.last();
        
        for n in nums {
            if last == None || *last.unwrap() < n {
                stack.push(n);
            } else {
                Self::exchange_with(&mut stack, n);
            }
            last = stack.last();
        }
        stack.len() as i32
    }    
}