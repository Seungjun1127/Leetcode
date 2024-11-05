impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let sum = |l: &Vec<i32>| {
            let mut v = 0;
            for i in 0..l.len() {
                v += l[i];
            }
            v
        };
        let mut max = sum(&accounts[0]);

        for i in 1..accounts.len() {
            let candidate = sum(&accounts[i]);
            if candidate > max {
                max = candidate;
            }
        }
        max
    }
}