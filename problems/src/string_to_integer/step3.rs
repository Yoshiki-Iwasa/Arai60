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

#[derive(Debug, Clone)]
enum Sign {
    Positive,
    Negative,
}

impl Solution {
    // 手順を分けて、checked_hogeを使って解くパターン
    pub fn my_atoi(s: String) -> i32 {
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
}
