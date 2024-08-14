// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - obstacleがあるところは0にして探索を進めればいい
    幅優先探索をしながらやっていく

    ちゃんとやるなら、gridを構造体として管理して、長方形が担保されるようにvalidationをかける

  想定ユースケース
  -

  正解してから気づいたこと
  - top rowとleft colを先に処理してfor loopする方法もある
*/

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
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
