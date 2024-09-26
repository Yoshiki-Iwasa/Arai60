// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  https://ja.wikipedia.org/wiki/%E3%82%AB%E3%82%BF%E3%83%A9%E3%83%B3%E6%95%B0
  時間計算量: O(Cn): カタラン数
            C_n \approx \frac{4^n}{n^{3/2} \sqrt{\pi}} らしいので
            O(4^n / n^(3/2) / sqrt(pi))
  空間計算量: 同上
*/

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        Self::create_valid_parentheses(n as usize, n as usize, &mut String::new())
    }

    fn create_valid_parentheses(
        rest_open_count: usize,
        rest_close_count: usize,
        parenthesis: &mut String,
    ) -> Vec<String> {
        if rest_open_count == 0 && rest_close_count == 0 {
            return vec![parenthesis.clone()];
        }

        let mut valid_parentheses = vec![];

        if rest_open_count > 0 {
            parenthesis.push('(');
            valid_parentheses.extend(Self::create_valid_parentheses(
                rest_open_count - 1,
                rest_close_count,
                parenthesis,
            ));
            parenthesis.pop();
        }

        if rest_close_count > 0 && rest_close_count > rest_open_count {
            parenthesis.push(')');
            valid_parentheses.extend(Self::create_valid_parentheses(
                rest_open_count,
                rest_close_count - 1,
                parenthesis,
            ));
            parenthesis.pop();
        }

        valid_parentheses
    }
}
