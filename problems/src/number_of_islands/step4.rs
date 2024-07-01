// Step4
// 目的: 指摘を修正し染み込ませる
pub struct Solution;

impl Solution {
    const VISITED: char = '2';
    const LAND: char = '1';
    const WATER: char = '0';
    pub fn num_islands_dfs(grid: Vec<Vec<char>>) -> i32 {
        let mut seen = grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut island_count = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if !seen[y][x] && grid[y][x] == Self::LAND {
                    island_count += 1;
                    Self::visit_connected_lands(&grid, &mut seen, y, x);
                }
            }
        }
        island_count
    }

    fn is_out_of_grid(grid: &[Vec<char>], y: usize, x: usize) -> bool {
        !(y < grid.len() && grid.get(y).is_some_and(|row| x < row.len()))
    }

    fn visit_connected_lands(grid: &[Vec<char>], seen: &mut [Vec<bool>], y: usize, x: usize) {
        if Self::is_out_of_grid(&grid, y, x) {
            return;
        }

        if seen[y][x] || grid[y][x] == Self::WATER {
            return;
        }

        seen[y][x] = true;

        for (next_y, next_x) in [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)] {
            Self::visit_connected_lands(grid, seen, next_y, next_x)
        }
    }
}
