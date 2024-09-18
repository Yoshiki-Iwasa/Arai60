// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量:
  空間計算量:
*/

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    // HashMapを使った解法
    pub fn length_of_longest_substring_1(s: String) -> i32 {
        let mut head = 0;
        let mut tail = 0;
        let chars = s.chars().collect::<Vec<_>>();
        let mut char_to_index = HashMap::<char, usize>::new();
        let mut ans = 0;

        while let Some(c) = chars.get(tail) {
            if let Some(duplicated_char_idx) = char_to_index.get(c) {
                head = head.max(duplicated_char_idx + 1)
            }
            char_to_index.insert(*c, tail);
            ans = ans.max(tail - head + 1);
            tail += 1
        }

        ans as i32
    }

    // HashSetを使った解法
    pub fn length_of_longest_substring_2(s: String) -> i32 {
        let chars = s.chars().collect::<Vec<_>>();
        let mut unique_chars = HashSet::<char>::new();
        let mut ans = 0;

        let mut head = 0;
        for tail in 0..chars.len() {
            while unique_chars.contains(&chars[tail]) {
                unique_chars.remove(&chars[head]);
                head += 1
            }
            unique_chars.insert(chars[tail]);
            ans = ans.max(tail - head + 1);
        }

        ans as i32
    }
}
