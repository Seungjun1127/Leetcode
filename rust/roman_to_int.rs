impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let roman_map = |c| match c {
            'I' => 1,
            'V' => 5,
            'X' => 10,
            'L' => 50,
            'C' => 100,
            'D' => 500,
            'M' => 1000,
            _ => 0,
        };

        let mut iter = s.chars().peekable();
        let mut ans = 0;
        
        while let Some(current) = iter.next() {
            let current = roman_map(current);
  
            if let Some(&next) = iter.peek() {
                let next = roman_map(next);
            
                if current < next {
                    ans -= current;
                } else {
                    ans += current;
                }
            } else {
                ans += current;
            }
        }
        ans
    }
}
