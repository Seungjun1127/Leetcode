impl Solution {
    fn _divide(dividend: i64, divisor: i64) -> i64 {
        let mut shifts = 0;
        while (dividend >> shifts) >= divisor {
            shifts += 1;
        }
        if shifts == 0 { return 0; }
        
        (1 << (shifts - 1)) + Self::_divide(dividend - (divisor << (shifts - 1)), divisor)
    }

    pub fn divide(dividend: i32, divisor: i32) -> i32 {
        if divisor == 0 { return i32::MAX; }
        if dividend == 0 { return 0; }
        if dividend == i32::MIN && divisor == -1 { return i32::MAX; }

        let is_negative = (dividend < 0) ^ (divisor < 0);
        let result = Self::_divide((dividend as i64).abs(), (divisor as i64).abs());

        if is_negative { -result as i32 } else { result as i32 }
    }
}