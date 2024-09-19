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
  - 最初問題文の意味がわからず
    ジグザグってそういうことかと理解

    実際の手作業をイメージすると、法則に従ってこのテーブルを作って、文字を埋める
    最後に右から左、上からしたに舐めて答えを出す
    テーブルの大きさがどのくらいになるかを見積もる
    a    g
    b  f h
    c e  i
    d    j
    fまでをひとかたまりと考えると、
    必要な横の長さX = (y - 1) * (N / (2Y - 2) + 1)

    この大きさを確保する
    あとは、ルールに従ってジグザグにテーブルを埋めて読むだけ



  想定ユースケース
  -

  正解してから気づいたこと
  - めっちゃ遅かったっぽい
    テーブル全部作らなくても良かった説がある

  - const EMPTY_CELL: char = ' ';
    としているが、空白含めてジグザグにしたくなったとき書き換えが必要になる
    Option::None とかで初期化するべきだった

  - result は with_capacityで初期化するべきだった
*/

pub struct Solution;

enum Direction {
    Down,
    Diagonal,
}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        const EMPTY_CELL: char = ' ';
        if num_rows == 1 {
            return s;
        }
        let y_len = num_rows as usize;
        let x_len = (y_len - 1) * (s.len() / (2 * y_len - 2) + 1);
        let mut zigzag_table = vec![vec![EMPTY_CELL; x_len]; y_len]; // rustはempty charがないので' 'で初期化

        let mut direction = Direction::Down;
        let mut x = 0;
        let mut y = 0;

        for c in s.chars() {
            zigzag_table[y][x] = c;
            match direction {
                Direction::Down => {
                    y += 1;
                }
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

        let mut result = String::new();
        (0..y_len).for_each(|y| {
            (0..x_len).for_each(|x| {
                if zigzag_table[y][x] != EMPTY_CELL {
                    result.push(zigzag_table[y][x]);
                }
            })
        });

        result
    }
}
