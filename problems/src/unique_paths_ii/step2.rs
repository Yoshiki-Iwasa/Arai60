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
  - https://github.com/nittoco/leetcode/pull/27/files
  - https://github.com/fhiyo/leetcode/pull/35/files
  - https://github.com/goto-untrapped/Arai60/pull/33#discussion_r1667236507


  自分は上と左の2列を先にloopで処理するほうが好きかも
  だいたい方針は同じで、変数名や下処理をループにいれるか否かの違いか

  BFSの回答、悪くないと思うんだけど書いてる人いないなあ


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 上と左の列を処理するときにtry_for_eachとControlFlowを使ってみた
    Rustはできるだけiteratorを繰り返しに使うのが慣例（iteratorをつかうオーバーヘッドなし）なので
    トライしてみた。（for_each()ではbreakできない）
    が、普通にforの方が読みやすいのでstep3はforで書く

    i, j

*/

use std::{collections::VecDeque, ops::ControlFlow};

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        const OBSTACLE: i32 = 1;
        assert!(!obstacle_grid.is_empty());
        assert!(!obstacle_grid[0].is_empty());

        let y_len = obstacle_grid.len();
        let x_len = obstacle_grid[0].len();

        let mut num_of_paths = vec![vec![0; x_len]; y_len];

        (0..y_len).try_for_each(|y| {
            if obstacle_grid[y][0] == OBSTACLE {
                return ControlFlow::Break(());
            };
            num_of_paths[y][0] = 1;
            ControlFlow::Continue(())
        });
        (0..x_len).try_for_each(|x| {
            if obstacle_grid[0][x] == OBSTACLE {
                return ControlFlow::Break(());
            }
            num_of_paths[0][x] = 1;
            ControlFlow::Continue(())
        });

        (1..y_len).for_each(|y| {
            (1..x_len).for_each(|x| {
                if obstacle_grid[y][x] == OBSTACLE {
                    return;
                }
                num_of_paths[y][x] += num_of_paths[y - 1][x] + num_of_paths[y][x - 1];
            })
        });

        num_of_paths[y_len - 1][x_len - 1]
    }

    // bfsで解く
    pub fn unique_paths_with_obstacles_bfs(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        const OBSTACLE: i32 = 1;
        assert!(!obstacle_grid.is_empty());
        assert!(!obstacle_grid[0].is_empty());

        let y_len = obstacle_grid.len();
        let x_len = obstacle_grid[0].len();
        let mut num_of_paths = vec![vec![0; x_len]; y_len];

        let mut queue = VecDeque::new();

        queue.push_back((0, 0));

        while let Some((y, x)) = queue.pop_front() {
            // obstacle or visited
            if obstacle_grid[y][x] == OBSTACLE || num_of_paths[y][x] > 0 {
                continue;
            }
            num_of_paths[y][x] = match (y.checked_sub(1), x.checked_sub(1)) {
                (None, None) => 1, // y == 0, x == 0
                (None, Some(left_x)) => num_of_paths[y][left_x],
                (Some(upper_y), None) => num_of_paths[upper_y][x],
                (Some(upper_y), Some(left_x)) => num_of_paths[y][left_x] + num_of_paths[upper_y][x],
            };

            if y + 1 < y_len {
                queue.push_back((y + 1, x));
            }
            if x + 1 < x_len {
                queue.push_back((y, x + 1));
            }
        }

        num_of_paths[y_len - 1][x_len - 1]
    }
}
