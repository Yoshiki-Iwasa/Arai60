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
  - https://github.com/fhiyo/leetcode/pull/46/files
    loopで書くのもやってみる
    一回 unsigned_nにbindしなくても書けそう
    n / 2じゃなくて n >> 1にしてる

  - https://github.com/nittoco/leetcode/pull/17/files
    n & 1で書くのもbitを見てる感じでかっこいい

    自分は初手でbit演算するのはイメージに上がってこなかった
    どちらを選ぶべきかとかあるんだろうか


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 余計なbidを排除しつつ、再帰で書いてみる
  - loopでも書く

  書いてみた感じ、recursiveのほうがこの場合は読みやすいのではなかろうか
*/

pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }

        match n.is_positive() {
            true => Self::pow_loop(x, n as u32),
            false => 1.0 / Self::pow_loop(x, n.unsigned_abs()),
        }

        // match n.is_positive() {
        //     true => Self::pow_recursive(x, n as u32),
        //     false => 1.0 / Self::pow_recursive(x, n.unsigned_abs()),
        // }
    }

    fn pow_loop(mut x: f64, mut n: u32) -> f64 {
        let mut powered = 1.0;
        while n != 0 {
            if n & 1 != 0 {
                powered *= x
            }
            n >>= 1;
            x *= x
        }

        powered
    }

    #[allow(unused)]
    fn pow_recursive(x: f64, n: u32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n & 1 == 0 {
            Self::pow_recursive(x * x, n >> 1)
        } else {
            Self::pow_recursive(x * x, n >> 1) * x
        }
    }
}
