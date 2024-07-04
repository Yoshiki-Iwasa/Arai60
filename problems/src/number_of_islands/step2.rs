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
  -

  他の人のコードを読んで考えたこと
  - https://github.com/TORUS0818/leetcode/pull/19/files#diff-7bf79a79d710574f7e4eeb463b14e234c2581ea863e9ffdea2c096abf6457f41R136-R137
    Visited, Unvisitedだけを管理するのに違和感を感じた
    対称性がないのではないだろうか
    初めてこのコードを読んだ人は、水の場合はどうするの？となりそう
  - https://github.com/TORUS0818/leetcode/pull/19/files#r1649826946
    gridの保護について詳細が説明されてわかりやすい






  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 一応、gridを改変するやり方もやってみる
    デバック性、可読性が下がるのであまりやりたくない
    gridが今どういう状態になっているのかをワーキングメモリに乗せながらコードを読まないといけないので読んでいて疲れそう
  - 方形じゃない入力があった場合でもクラッシュしないような工夫が必要かも
*/

use std::{collections::VecDeque, usize};

pub struct Solution;
impl Solution {
    const VISITED: char = '2';
    const LAND: char = '1';
    const WATER: char = '0';

    // dfs, 訪問済みを別配列でもつパターン
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut island_count = 0;

        let mut seen = grid
            .iter()
            .map(|row| row.iter().map(|_| false).collect::<Vec<_>>())
            .collect::<Vec<_>>();

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                // 探索開始は島のときだけでOK
                if grid[y][x] == Self::LAND {
                    if Self::is_new_island(&mut seen, &grid, x, y) {
                        island_count += 1
                    }
                }
            }
        }

        island_count as i32
    }

    fn is_within_grid(grid: &[Vec<char>], y: &usize, x: &usize) -> bool {
        *y < grid.len() && *x < grid.get(*y).unwrap().len()
    }

    // あえてis_within_grid()とそれ以降の基底条件を分けている
    // index accessの安全性を確かめてからindexアクセスさせるため
    fn is_new_island(seen: &mut [Vec<bool>], grid: &[Vec<char>], y: usize, x: usize) -> bool {
        if !Self::is_within_grid(grid, &y, &x) {
            return false;
        }

        if seen[y][x] || grid[y][x] == Self::WATER {
            return false;
        }

        seen[y][x] = true;

        for (next_y, next_x) in [(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)] {
            if !Self::is_new_island(seen, grid, next_y, next_x) {
                continue;
            }
        }

        true
    }

    // BFSで解くパターン
    // 新しい土地を見つけたら increment
    // そこから辿れる土地を訪問済みにしていく
    // gridを書き換える方針でやってみる
    pub fn num_islands_bfs(mut grid: Vec<Vec<char>>) -> i32 {
        let mut island_count = 0;

        for y in 0..grid.len() {
            for x in 0..grid[y].len() {
                if grid[y][x] == Self::LAND {
                    island_count += 1;

                    // bfsで連結した土地を訪問済みにする
                    let mut queue = VecDeque::new();
                    queue.push_back((y, x));
                    while !queue.is_empty() {
                        // while 条件からunwrapは安全
                        let (y, x) = queue.pop_front().unwrap();
                        if !Self::is_within_grid(&grid, &y, &x) {
                            continue;
                        }

                        if grid[y][x] != Self::LAND {
                            continue;
                        }
                        grid[y][x] = Self::VISITED;

                        for (next_y, next_x) in [(y, x - 1), (y, x + 1), (y - 1, x), (y + 1, x)] {
                            queue.push_back((next_y, next_x))
                        }
                    }
                }
            }
        }

        island_count
    }
}

// 以下、union findを用いた解法
// 木構造を使ってグループ分けを高速に行うデータ構造
// 同じ根に属するノードは同じグループという発想で判定を行う

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
