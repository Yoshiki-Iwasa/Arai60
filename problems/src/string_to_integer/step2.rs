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
  - https://github.com/fhiyo/leetcode/pull/57/files
    C++でprimitiveな処理だけでやっている
    多分、これが書けるかを見られる`if (abs_value > INT_MAX / 10 || abs_value == INT_MAX / 10 && digit > INT_MAX % 10)`
    1 passで書いているけど、実際の仕事だったら文字列の正規化部分と変換部分は分けたい気がする
    全部一緒だとテストしづらいし、手作業でこれをやることを考えたら、一旦有効な部分だけを切り取って考えたいと思う

  - https://github.com/rihib/leetcode/pull/10/files#diff-42c20d3767f19520a902d52c56f13e2190d83f1077bda9840504997e6aa95659R31-R36
    Goを読み慣れていないせいか、各ステップで何をしているのかがわかりづらかった
    素直に書くなら、signで分けるか

  - https://github.com/goto-untrapped/Arai60/pull/9/files









  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - signを取得するとき、next_if()を使って短くする
  - skip_while('0') で余計な0が入らないようにする
  - take_while() -> map() を map_while()で省略する
  - current_integer -> predecessor_integer 二変更
  - digitsの畳込みを1行で書いた。コメントで意図を補足したが、これはコメント付きそう

  - 1 passでchecked_hogeを使わずに書く
*/

pub struct Solution;

#[derive(Debug, Clone)]
enum Sign {
    Positive,
    Negative,
}

impl Solution {
    // 手順を分けて、checked_hogeを使って解くパターン
    pub fn my_atoi_1(s: String) -> i32 {
        let (sign, digits) = Self::normalize(s);
        Self::into_i32(sign, digits)
    }

    // separate input into sign and valid digits
    fn normalize(s: String) -> (Sign, Vec<u8>) {
        let s = s.trim(); // trim leading and ending whitespace
        let mut chars = s.chars().peekable();

        let sign = match chars.next_if(|&c| c == '-' || c == '+') {
            Some(sign) => {
                if sign == '-' {
                    Sign::Negative
                } else {
                    Sign::Positive
                }
            }
            None => Sign::Positive,
        };

        let digits = chars
            .skip_while(|&c| c == '0') // trim leading 0
            .map_while(|c| c.to_digit(10).map(|n| n as u8)) // filter in leading digits
            .collect::<Vec<_>>();
        (sign, digits)
    }

    fn into_i32(sign: Sign, digits: Vec<u8>) -> i32 {
        digits
            .into_iter()
            .fold(0_i32, |predecessor_integer, n| match sign {
                // Round to i32::MAX or i32::MIN if overflow occurs
                Sign::Positive => predecessor_integer
                    .checked_mul(10)
                    .unwrap_or(i32::MAX)
                    .checked_add(n as i32)
                    .unwrap_or(i32::MAX),
                Sign::Negative => predecessor_integer
                    .checked_mul(10)
                    .unwrap_or(i32::MIN)
                    .checked_sub(n as i32)
                    .unwrap_or(i32::MIN),
            })
    }

    // 1 passで、checked_hogeを使わないで書いてみる
    pub fn my_atoi_2(s: String) -> i32 {
        let s = s.trim();
        let mut chars = s.chars().peekable();

        fn is_sign(&c: &char) -> bool {
            c == '-' || c == '+'
        }

        let sign = match chars.next_if(is_sign) {
            Some(c) => {
                if c == '-' {
                    Sign::Negative
                } else {
                    Sign::Positive
                }
            }
            None => Sign::Positive,
        };

        // trim leading zero
        while chars.peek().is_some_and(|&c| c == '0') {
            chars.next();
        }

        let mut result = 0_i32;
        for c in chars {
            let Some(num) = c.to_digit(10) else {
                break;
            };
            if result > i32::MAX / 10 || result == i32::MAX / 10 && num as i32 > i32::MAX % 10 {
                match sign {
                    Sign::Positive => return i32::MAX,
                    Sign::Negative => return i32::MIN,
                }
            }

            result *= 10;
            result += num as i32
        }

        match sign {
            Sign::Positive => result,
            Sign::Negative => -result,
        }
    }
}
