use std::collections::HashMap;

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

        for (next_y, next_x) in [(y + 1, x), (y, x + 1)] {
            Self::visit_connected_lands(grid, seen, next_y, next_x)
        }
    }
}

pub struct UnionFind {
    roots: Vec<usize>,
    ranks: Vec<usize>,
    root_count: u32,
}

fn into_1_dimension_index(y: usize, x: usize, x_len: usize) -> usize {
    y * x_len + x
}

impl UnionFind {
    pub fn from_grid(grid: &[Vec<char>]) -> Self {
        let mut roots = vec![];
        let mut ranks = vec![];
        let mut root_count = 0;
        let x_len = grid[0].len();
        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == '1' {
                    roots.push(into_1_dimension_index(y, x, x_len));
                    root_count += 1;
                } else {
                    roots.push(usize::MAX)
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

    pub fn root(&mut self, elem: usize) -> usize {
        if self.roots[elem] != elem {
            self.roots[elem] = self.root(self.roots[elem])
        }
        self.roots[elem]
    }

    pub fn unite(&mut self, a: usize, b: usize) {
        let root_a = self.root(a);
        let root_b = self.root(b);

        if root_a == root_b {
            return;
        }

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
                if grid[y][x] == Self::WATER {
                    continue;
                }
                grid[y][x] = Self::VISITED;
                for (next_y, next_x) in [(y, x + 1), (y + 1, x)] {
                    if Self::is_out_of_grid(&grid, next_y, next_x) {
                        continue;
                    }

                    if grid[next_y][next_x] != Self::LAND {
                        continue;
                    }

                    union_find.unite(y * x_len + x, next_y * x_len + next_x);
                }
            }
        }

        union_find.get_root_count() as i32
    }
}
