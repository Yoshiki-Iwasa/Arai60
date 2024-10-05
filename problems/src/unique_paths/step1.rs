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
  - 後戻りできないので、マスy.xに行く方法はy-1,x と y, x-1に行く方法の和になる

  想定ユースケース
  -

  正解してから気づいたこと
  - y = 0, x = 0 の部分は1と決まっているのだから、1で初期化して更新すればよかった
*/

use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        // m is row, n is col
        let m = m as usize;
        let n = n as usize;

        let mut num_of_paths = vec![vec![0; m]; n];
        let mut visited = vec![vec![false; m]; n];

        let mut queue = VecDeque::new();
        queue.push_back((0, 0));

        while let Some((y, x)) = queue.pop_front() {
            if visited[y][x] {
                continue;
            }
            visited[y][x] = true;

            num_of_paths[y][x] = match (y.checked_sub(1), x.checked_sub(1)) {
                // y == 0, x == 0
                (None, None) => 1,
                (None, Some(left_x)) => num_of_paths[y][left_x],
                (Some(upper_y), None) => num_of_paths[upper_y][x],
                (Some(upper_y), Some(left_x)) => num_of_paths[y][left_x] + num_of_paths[upper_y][x],
            };
            if y + 1 < n {
                queue.push_back(((y + 1), x));
            }
            if x + 1 < m {
                queue.push_back((y, (x + 1)));
            }
        }

        num_of_paths[n - 1][m - 1]
    }
}
