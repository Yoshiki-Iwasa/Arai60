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
  - 浮動小数を扱うときはドキッとする
    nが負の場合、毎回割り算をするよりも最後に一発だけやったほうが精度が高くでるのでそうする

    -2^31 <= n <= 2^31-1
    -100.0 < x < 100.0
    なので、f64に収まるんだっけ
    f64::MAX = 1.7976931348623157E+308
    f64::MIN = -1.7976931348623157E+308
    なのでキビくない？

    -10^4 <= x^n <= 10^4
    という条件をみつけてホッとする

  - 再帰だとn = 2^31-1の時、overflowしそうなのでloopでかく
    nのオーダーが10^10くらいになるからlogで処理する必要あり
    2 * 2 * 2 * 2 => 4 * 4 みたいにすればlogでいけそう
    なら再帰でもいいか

  - https://github.com/rust-lang/rust/blob/master/library/core/src/num/int_macros.rs#L2157
    unsigned_abs()はwrapped_abs()を内部的に呼び出している
    i32::MINのときだけwrapped_abs()からi32::MINが返ってくるが as u32でキャストされるため問題なし
    このキャスト時、u32にbitをコピーしたあとその補数表現が取られる（https://doc.rust-jp.rs/rust-by-example-ja/types/cast.html）
    なので安全にキャストできる

  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }

        let unsigned_n: u32 = n.unsigned_abs();

        let powered = Self::pow(x, unsigned_n);

        match n.is_positive() {
            true => powered,
            false => 1.0 / powered,
        }
    }

    fn pow(x: f64, n: u32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n % 2 == 0 {
            Self::pow(x * x, n / 2)
        } else {
            Self::pow(x * x, n / 2) * x
        }
    }
}
