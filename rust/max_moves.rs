impl Solution {
    fn max_moves_rec(grid: &Vec<Vec<i32>>, row: usize, col: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if grid.len() == 0 || grid[0].len() == 0 {
            return 0;
        }

        let height = grid.len();
        let width = grid[0].len();

        if row >= height || col >= width {
            return 0;
        }

        if memo[row][col] != -1 {
            return memo[row][col];
        }

        let current = grid[row][col];
        let mut max_move = 0;

        // Define a helper closure to calculate next moves
        let mut next = |next_row: usize, next_col: usize| -> i32 {
            if next_row < height && next_col < width && grid[next_row][next_col] > current {
                1 + Self::max_moves_rec(grid, next_row, next_col, memo)
            } else {
                0
            }
        };

        // Check moves in the three allowed directions
        max_move = max_move
            .max(next(row.saturating_sub(1), col + 1))
            .max(next(row, col + 1))
            .max(next(row + 1, col + 1));

        // Memoize result
        memo[row][col] = max_move;
        memo[row][col]
    }

    pub fn max_moves(grid: Vec<Vec<i32>>) -> i32 {
        if grid.is_empty() || grid[0].is_empty() {
            return 0;
        }
        let (row, col) = (grid.len(), grid[0].len());
        let mut memo = vec![vec![-1; col]; row];

        (0..row).map(|i| Self::max_moves_rec(&grid, i, 0, &mut memo)).max().unwrap_or(0)
    }
}
