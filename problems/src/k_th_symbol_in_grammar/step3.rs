// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  一番右端にkがある場合
  時間計算量: O(log k)
  空間計算量: O(log k)
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
            Self::kth_grammar(n - 1, k - row_len / 2) ^ 1
        }
    }

    pub fn kth_grammar_2(mut n: i32, mut k: i32) -> i32 {
        let mut kth_symbol = 0;

        while k > 1 {
            let row_len = 2_i32.pow((n - 1) as u32);
            if k > row_len / 2 {
                k -= row_len / 2;
                kth_symbol ^= 1
            }
            n -= 1
        }

        kth_symbol
    }
}
