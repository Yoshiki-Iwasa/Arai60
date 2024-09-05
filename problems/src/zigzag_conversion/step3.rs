// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(N)
  空間計算量: O(N)
*/

pub struct Solution;

enum Direction {
    Down,
    Diagonal,
}

impl Solution {
    // rowに入ってる文字列を表す配列をいじるパターン
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }

        let num_rows = num_rows as usize;

        let mut rows = vec![String::new(); num_rows];
        let mut direction = Direction::Down;
        let mut row_index = 0;

        s.chars().for_each(|c| {
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
        });

        rows.into_iter().collect::<String>()
    }
}
