impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
		let row = matrix.len();
		let col = matrix[0].len();
        let mut max = 0;

		let cumul_AND = |start: usize, height: usize| -> Vec<char> {
			let mut res = Vec::new();
			for j in 0..col {
				if (start..start+height).all(|i| matrix[i][j] == '1') {
					res.push('1');
				} else {
					res.push('0');
				}
			}
			res
		};

		let maximun_continue = |vec: Vec<char>| {
			let mut max = 0;
			let mut len = 0;
			for i in 0..vec.len() {
				if vec[i] == '1' {
					len += 1;
				} else {
					if max < len {
						max = len;
					}
					len = 0;
				}
			};

			len.max(max)
		};

		for height in 1..=row {
			for i in 0..=(row - height) {
				let lect = maximun_continue(cumul_AND(i, height));
				max = max.max(lect * height);
			}
		}
		max as i32
    }
}