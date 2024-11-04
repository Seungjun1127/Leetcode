impl Solution {
    pub fn maximum_swap(num: i32) -> i32 {
        let mut digits: Vec<char> = num.to_string().chars().collect();
        let mut last_occurrence = vec![-1; 10];

        // 각 숫자가 마지막으로 나타난 위치 기록
        for i in 0..digits.len() {
            let digit = digits[i].to_digit(10).unwrap() as usize;
            last_occurrence[digit] = i as i32;
        }

        // 자릿수를 순회하면서 교환할 두 자리를 찾음
        for i in 0..digits.len() {
            let current_digit = digits[i].to_digit(10).unwrap() as usize;
            // 더 큰 숫자가 뒤쪽에 있는지 확인
            for d in (current_digit + 1..=9).rev() {
                if last_occurrence[d] > i as i32 {
                    // 교환 후 즉시 최대 숫자 반환
                    digits.swap(i, last_occurrence[d] as usize);
                    return digits.iter().collect::<String>().parse().unwrap();
                }
            }
        }

        // 교환할 수 없다면 원래 숫자 반환
        num
    }
}