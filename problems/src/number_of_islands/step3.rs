// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量:
  空間計算量:
*/

use std::collections::VecDeque;

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
                if grid[y][x] == Self::LAND {
                    if Self::is_new_island(&grid, &mut seen, y, x) {
                        island_count += 1;
                    }
                }
            }
        }
        island_count
    }

    fn is_within_grid(grid: &[Vec<char>], y: &usize, x: &usize) -> bool {
        *y < grid.len() && grid.get(*y).is_some_and(|row| *x < row.len())
    }

    fn is_new_island(grid: &[Vec<char>], seen: &mut [Vec<bool>], y: usize, x: usize) -> bool {
        if !Self::is_within_grid(grid, &y, &x) {
            return false;
        }

        if seen[y][x] || grid[y][x] == Self::WATER {
            return false;
        }

        seen[y][x] = true;

        for (next_y, next_x) in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)] {
            if !Self::is_new_island(grid, seen, next_y, next_x) {
                continue;
            }
        }

        true
    }

    pub fn num_islands_bfs(mut grid: Vec<Vec<char>>) -> i32 {
        let mut island_count = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Self::LAND {
                    island_count += 1;
                    let mut queue = VecDeque::new();
                    queue.push_back((y, x));
                    while !queue.is_empty() {
                        let (y, x) = queue.pop_front().unwrap();

                        if !Self::is_within_grid(&grid, &y, &x) {
                            continue;
                        }

                        if grid[y][x] != Self::LAND {
                            continue;
                        }

                        grid[y][x] = Self::VISITED;

                        for (next_y, next_x) in [(y, x + 1), (y, x - 1), (y + 1, x), (y - 1, x)] {
                            queue.push_back((next_y, next_x))
                        }
                    }
                }
            }
        }

        island_count
    }
}

pub struct UnionFind {
    roots: Vec<usize>,
    ranks: Vec<usize>,
    root_count: u32,
}

impl UnionFind {
    pub fn from_grid(grid: &[Vec<char>]) -> Self {
        let mut roots = vec![];
        let mut ranks = vec![];
        let mut root_count = 0;
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '1' {
                    // 2次元配列のindexを一次元配列のindexとして表現
                    roots.push(y * grid[y].len() + x);
                    root_count += 1;
                } else {
                    // 32bitマシンならu32::MAX, 64bitマシンならu64::MAX
                    roots.push(usize::MAX);
                }
                ranks.push(0)
            }
        }
        Self {
            roots,
            ranks,
            root_count,
        }
    }

    pub fn root(&mut self, num: usize) -> usize {
        if self.roots[num] != num {
            // pass compression
            self.roots[num] = self.root(self.roots[num]);
        }
        self.roots[num]
    }

    pub fn unite(&mut self, a: usize, b: usize) {
        let root_a = self.root(a);
        let root_b = self.root(b);

        if root_a != root_b {
            if self.ranks[root_a] > self.ranks[root_b] {
                self.roots[root_b] = root_a
            } else {
                self.roots[root_a] = root_b;
                if self.ranks[root_a] == self.ranks[root_b] {
                    self.ranks[root_a] += 1;
                }
            }
            self.root_count -= 1;
        }
    }

    pub fn is_same_group(&mut self, a: usize, b: usize) -> bool {
        self.root(a) == self.root(b)
    }

    pub fn get_root_count(&self) -> u32 {
        self.root_count
    }
}

impl Solution {
    pub fn num_islands_union_find(mut grid: Vec<Vec<char>>) -> i32 {
        let mut union_find = UnionFind::from_grid(&grid);
        let y_len = grid.len();
        let x_len = grid[0].len();

        for y in 0..y_len {
            for x in 0..x_len {
                if grid[y][x] == Self::LAND {
                    grid[y][x] = Self::VISITED;
                    for (next_y, next_x) in [(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)] {
                        if !Self::is_within_grid(&grid, &next_y, &next_x) {
                            continue;
                        }

                        if grid[next_y][next_x] != Self::LAND {
                            continue;
                        }

                        union_find.unite(y * x_len + x, next_y * x_len + next_x);
                    }
                }
            }
        }

        union_find.get_root_count() as i32
    }
}
