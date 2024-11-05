impl Solution {
    pub fn check_if_exist(arr: Vec<i32>) -> bool {
        let mut set = vec![false; 2001];
        let mut zero_cnt = 0;

        for &n in arr.iter() {
            if n == 0 {
                zero_cnt += 1;
            }
            set[(n + 1000) as usize] = true;
        }
        if zero_cnt > 1 {
            return true;
        }

        for &n in arr.iter() {
            let doub = 2 * n;

            if doub < -1000 || doub > 1000 || doub == 0 {
                continue;
            }

            if set[(doub + 1000) as usize] {
                print!("{} exist", doub);
                return true;
            }
        }

        false
    }
}