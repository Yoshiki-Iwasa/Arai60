// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  N: target.len()
  M: word_dict.len()
  K: average of the length of words in word_dict
  時間計算量: O(M * K * N / K) = O(M * N)
  空間計算量: O(M)
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        Self::word_break_helper(s.as_str(), &word_dict, &mut HashMap::new())
    }

    fn word_break_helper<'a>(
        target: &'a str,
        word_dict: &[String],
        memo: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if let Some(result) = memo.get(target) {
            return *result;
        }
        if target.is_empty() {
            return true;
        }

        for word in word_dict.iter() {
            if target.starts_with(word) {
                match Self::word_break_helper(&target[word.len()..], word_dict, memo) {
                    true => {
                        memo.insert(target, true);
                        return true;
                    }
                    false => continue,
                }
            }
        }
        memo.insert(target, false);
        false
    }
}
