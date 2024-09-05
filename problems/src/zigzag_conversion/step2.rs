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
  - https://github.com/fhiyo/leetcode/pull/58/files#diff-9924eba75ce80cfb26b487ccdaaf7bae5d8affe8661b63382ef4051a602e81f6R16
    テーブルをすべて作る必要なかった
    rowを上下に移動しながら文字列つくって最後に足し合わせれば十分だった

  - https://github.com/fhiyo/leetcode/pull/58/files#diff-9924eba75ce80cfb26b487ccdaaf7bae5d8affe8661b63382ef4051a602e81f6R87
    ジグザグに動くのではなく、indexを動かしてresultを作りに行く

  - https://github.com/Mike0121/LeetCode/pull/26
  - https://github.com/Exzrgs/LeetCode/pull/3/files




  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - テーブルの空きはNoneで表現することにする
  -

  - テーブルを作らずにrowのStringを管理する方法でやってみる
*/

pub struct Solution;

enum Direction {
    Down,
    Diagonal,
}

impl Solution {
    // テーブルを作ってジグザグに埋める
    pub fn convert_1(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let y_len = num_rows as usize;
        let x_len = (y_len - 1) * (s.len() / (2 * y_len - 2) + 1);
        let mut zigzag_table: Vec<Vec<Option<char>>> = vec![vec![None; x_len]; y_len];

        let mut direction = Direction::Down;
        let mut x = 0;
        let mut y = 0;

        for c in s.chars() {
            zigzag_table[y][x] = Some(c);
            match direction {
                Direction::Down => y += 1,
                Direction::Diagonal => {
                    y -= 1;
                    x += 1;
                }
            }
            if y == 0 {
                direction = Direction::Down
            }
            if y == y_len - 1 {
                direction = Direction::Diagonal
            }
        }

        let mut result = String::with_capacity(s.len());
        (0..y_len).for_each(|y| {
            (0..x_len).for_each(|x| {
                if let Some(c) = zigzag_table[y][x] {
                    result.push(c);
                }
            })
        });

        result
    }

    // rowに入ってる文字列を表す配列をいじるパターン
    pub fn convert_2(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let num_rows = num_rows as usize;
        let mut rows = vec![String::new(); num_rows];

        let mut direction = Direction::Down;
        let mut row_index = 0;
        for c in s.chars() {
            rows[row_index].push(c);
            match direction {
                Direction::Down => row_index += 1,
                Direction::Diagonal => row_index -= 1,
            };
            if row_index == 0 {
                direction = Direction::Down
            }
            if row_index == num_rows - 1 {
                direction = Direction::Diagonal
            }
        }

        rows.into_iter().collect::<String>()
    }
}
