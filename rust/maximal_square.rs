impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        let row = matrix.len();
		let col = matrix[0].len();
		let mut dp = vec![vec![0; col]; row];
		let mut max = 0;

		for i in 0..row {
			for j in 0..col {
				if matrix[i][j] == '1' {
					if i == 0 || j == 0 {
						dp[i][j] = 1;
					} else {
						dp[i][j] = 1 + dp[i-1][j-1].min(dp[i-1][j]).min(dp[i][j-1]);
					}
					
                    if max < dp[i][j] {
                        max = dp[i][j];
                    }
				}
			}
		}
		max * max
    }
}
