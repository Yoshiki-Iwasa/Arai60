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
  - stringをi32に変換
    なんとなく、いろいろ注意しないといけないことがありそう
    入力サイズは200か、9 * 200が最大か

  - 実装方針
    正規化パートと変換パートで分ける
    まず、入力を条件に従って正規化。(is_positive: bool, digits: Vec<char>) みたいに分ける

    変換パートでは、i32の範囲内に収まるかを確認しながら桁を上げて足していく
    途中でi32の範囲外に出たら丸めてreturn

  - 手続き的に書く方針と,「Stringで表現された整数」を定義してそこからのinto()で解く方法がある

  15分くらいで1発で通せたが、rust-analyzerなしだったらもうちょいかかってた気がする
  標準ライブラリの関数の戻り値の型を覚えてないのがちらほら

  想定ユースケース
  -

  正解してから気づいたこと
  - `next_if()`という関数があったからこれを使えばpeekableにしなくても良かった

  - signを取った後、skip_while(|c| c == '0')で飛ばすのもあり

  - 桁を上げて足していく部分は、1行にまとめることもできる

  - 面接で出題されるとしたら、その意図は整数の四則演算をオーバーフローしないように扱えますか？だろう
    Rustのchecked_hoge() を使わないでprimitiveな計算のみでStep2ではやってみる

    iteratorによる処理は別にいいか。

  - 面接でこれ出たら、
*/

pub struct Solution;

#[derive(Debug, Clone)]
enum Sign {
    Positive,
    Negative,
}

impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let (sign, digits) = Self::normalize(s);
        Self::into_i32(sign, digits)
    }

    fn normalize(s: String) -> (Sign, Vec<u8>) {
        let s = s.trim(); // trim leading and ending whitespace
        let mut peekable_chars = s.chars().peekable();

        let sign = match peekable_chars.peek() {
            Some(c) => match c {
                '-' => {
                    peekable_chars.next(); // consume sign
                    Sign::Negative
                }
                '+' => {
                    peekable_chars.next(); // consume sign
                    Sign::Positive
                }
                _ => Sign::Positive,
            },
            None => return (Sign::Positive, vec![]),
        };
        let digits = peekable_chars
            .take_while(|c| c.is_ascii_digit())
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect::<Vec<_>>();
        (sign, digits)
    }

    fn into_i32(sign: Sign, digits: Vec<u8>) -> i32 {
        digits
            .into_iter()
            .fold(0_i32, |current_integer, n| match sign {
                Sign::Positive => {
                    let Some(new_integer) = current_integer.checked_mul(10) else {
                        return i32::MAX;
                    };
                    new_integer.checked_add(n as i32).unwrap_or(i32::MAX)
                }
                Sign::Negative => {
                    let Some(new_integer) = current_integer.checked_mul(10) else {
                        return i32::MIN;
                    };
                    new_integer.checked_sub(n as i32).unwrap_or(i32::MIN)
                }
            })
    }
}
