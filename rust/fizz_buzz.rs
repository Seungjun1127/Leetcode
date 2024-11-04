impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let fizz_map = |i: i32| match (i % 3, i % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            (_, _) => i.to_string(),
        };

        let mut ans = Vec::new();
        for i in 1..=n {
            ans.push(fizz_map(i));
        }
        ans
    }
}