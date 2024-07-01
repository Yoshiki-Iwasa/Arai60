// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  - land_countをmutable referenceとして引き回さない手はないか

  他の人のコードを読んで考えたこと
  - https://github.com/TORUS0818/leetcode/pull/20#pullrequestreview-2146362384
    Union Findの実装はやはり大変（量が多い）
    競プロのライブラリとして使うのはいいけど、面接では実装しないほうがよさそう
    とはいえ書けるのは大切
  - https://github.com/goto-untrapped/Arai60/pull/31/files#diff-85dc3b14103f7b0e5c70b4479d22dd5eeb3e1b2df81b811455a69901cffb2e8e
    範囲外アクセスを防ぐなら、`is_out_of_range`の方が適切そう


  他の想定ユースケース
  - 全国のもっとも駅の数が多い路線の駅数を返すとき
    その場合、再帰だとスタックが深くなりすぎるので、iterativeな方法でやる


  改善する時にかんがえたこと
  - Rustは再帰が深くなるとoverflowする言語なので、iterativeにも書けるようにしておく
*/

use std::collections::VecDeque;

pub struct Solution;
impl Solution {
    const LAND: i32 = 1;
    const WATER: i32 = 0;
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut seen = grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut max_land_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Self::LAND {
                    max_land_count = max_land_count.max(Self::count_lands(&grid, &mut seen, y, x));
                }
            }
        }

        max_land_count as i32
    }

    fn is_out_of_grid(grid: &[Vec<i32>], y: &usize, x: &usize) -> bool {
        !(*y < grid.len() && grid.get(*y).is_some_and(|row| *x < row.len()))
    }

    pub fn count_lands(grid: &[Vec<i32>], seen: &mut [Vec<bool>], y: usize, x: usize) -> i32 {
        if Self::is_out_of_grid(grid, &y, &x) {
            return 0;
        }

        if seen[y][x] || grid[y][x] == Self::WATER {
            return 0;
        }

        seen[y][x] = true;

        let mut land_count = 1;

        for (next_y, next_x) in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)] {
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
                    let mut land_count = 0;
                    stack.push_front((y, x));
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

                        for (next_y, next_x) in [(y + 1, x), (y - 1, x), (y, x + 1), (y, x - 1)] {
                            stack.push_front((next_y, next_x));
                        }
                    }

                    max_land_count = max_land_count.max(land_count);
                }
            }
        }

        max_land_count
    }
}
