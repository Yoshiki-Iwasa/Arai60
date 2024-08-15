// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  N: grid数
  時間計算量: O(N)
  空間計算量: O(N)
*/

pub struct Solution;

impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        const OBSTACLE: i32 = 1;
        assert!(!obstacle_grid.is_empty());
        assert!(!obstacle_grid[0].is_empty());

        let y_len = obstacle_grid.len();
        let x_len = obstacle_grid[0].len();

        let mut num_of_paths = vec![vec![0; x_len]; y_len];

        for y in 0..y_len {
            if obstacle_grid[y][0] == OBSTACLE {
                break;
            }
            num_of_paths[y][0] = 1;
        }

        for x in 0..x_len {
            if obstacle_grid[0][x] == OBSTACLE {
                break;
            }
            num_of_paths[0][x] = 1;
        }

        (1..y_len).for_each(|y| {
            (1..x_len).for_each(|x| {
                if obstacle_grid[y][x] == OBSTACLE {
                    return;
                }
                num_of_paths[y][x] = num_of_paths[y - 1][x] + num_of_paths[y][x - 1];
            });
        });

        num_of_paths[y_len - 1][x_len - 1]
    }
}
