// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n)
  空間計算量: O(n)
*/

use std::intrinsics::mir::UnwindContinue;

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = vec![];

        for c in s.chars() {
            let Some(top) = stack.last() else {
                stack.push(c);
                continue;
            };

            let is_match = match (top, c) {
                ('(', ')') => true,
                ('{', '}') => true,
                ('[', ']') => true,
                _ => false,
            };

            if is_match {
                stack.pop();
            } else {
                stack.push(c);
            }
        }
        stack.is_empty()
    }
}
