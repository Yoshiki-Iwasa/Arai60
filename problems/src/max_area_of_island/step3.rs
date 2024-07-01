// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

use std::collections::VecDeque;

/*
  時間計算量: O(H * W)
  空間計算量: O(H * W)
*/
pub struct Solution;
impl Solution {
    const WATER: i32 = 0;
    const LAND: i32 = 1;

    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut max_land_count = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Self::LAND {
                    max_land_count =
                        std::cmp::max(max_land_count, Self::count_lands(&grid, &mut seen, y, x));
                }
            }
        }

        max_land_count
    }

    fn is_out_of_grid(grid: &[Vec<i32>], y: &usize, x: &usize) -> bool {
        !(*y < grid.len() && grid.get(*y).is_some_and(|row| *x < row.len()))
    }

    fn count_lands(grid: &[Vec<i32>], seen: &mut [Vec<bool>], y: usize, x: usize) -> i32 {
        if Self::is_out_of_grid(grid, &y, &x) {
            return 0;
        }

        if seen[y][x] || grid[y][x] == Self::WATER {
            return 0;
        }

        seen[y][x] = true;
        let mut land_count = 1;

        for (next_y, next_x) in [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)] {
            land_count += Self::count_lands(grid, seen, next_y, next_x)
        }

        land_count
    }

    pub fn max_area_of_island_iterative(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut max_land_count = 0;

        let mut stack = VecDeque::new();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Self::LAND {
                    stack.push_front((y, x));

                    let mut land_count = 0;
                    while !stack.is_empty() {
                        let (y, x) = stack.pop_front().unwrap();

                        if Self::is_out_of_grid(&grid, &y, &x) {
                            continue;
                        }

                        if seen[y][x] || grid[y][x] == Self::WATER {
                            continue;
                        }

                        seen[y][x] = true;
                        land_count += 1;

                        for next in [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)] {
                            stack.push_front(next)
                        }
                    }
                    max_land_count = std::cmp::max(max_land_count, land_count);
                }
            }
        }

        max_land_count
    }
}
