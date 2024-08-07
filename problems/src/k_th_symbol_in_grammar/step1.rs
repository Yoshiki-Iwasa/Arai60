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
  - n = 5くらいまで書き出すと法則が見えてきた
    f(n) = f(n - 1) append !f(n - 1) だった
    f(n)のk番目が知りたい場合:
      k <= len(f(n)) / 2なら f(n-1)のk番目を返す
      k > len(f(n)) / 2なら f(n-1)の k - len(f(n-1))を反転させた値を返す
      基底条件は k == 1 の時0を返すこと

      これを繰り返していけばlog nで行ける



  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }

        let row_len = 2_i32.pow((n - 1) as u32);

        if k <= row_len / 2 {
            Self::kth_grammar(n - 1, k)
        } else {
            Self::kth_grammar(n - 1, k - row_len / 2) ^ 1 // XOR
        }
    }
}
