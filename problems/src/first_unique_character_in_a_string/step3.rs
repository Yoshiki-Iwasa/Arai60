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
  空間計算量: O(1) *
*/

use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        const NOT_FOUND: i32 = -1;
        const INVALID_INPUT: i32 = -1;
        const NUMBER_OF_ALPHABETS: usize = 26;

        if s.chars().any(|c| !c.is_ascii_lowercase()) {
            return INVALID_INPUT;
        }

        let mut char_histogram = [0_u32; NUMBER_OF_ALPHABETS];

        let offset = 'a' as usize;

        for c in s.chars() {
            char_histogram[c as usize - offset] += 1;
        }

        for (index, c) in s.char_indices() {
            if char_histogram[c as usize - offset] == 1 {
                return index as i32;
            }
        }

        NOT_FOUND
    }

    pub fn first_uniq_char_hash_map(s: String) -> i32 {
        const NOT_FOUND: i32 = -1;
        const INVALID_INPUT: i32 = -2;
        const NUMBER_OF_ALPHABETS: usize = 26;
        let mut char_count = HashMap::<char, usize>::with_capacity(NUMBER_OF_ALPHABETS);

        for c in s.chars() {
            if !c.is_ascii_lowercase() {
                return INVALID_INPUT;
            }
            *char_count.entry(c).or_insert(0) += 1;
        }

        for (index, c) in s.char_indices() {
            if char_count.get(&c).is_some_and(|cnt| *cnt == 1) {
                return index as i32;
            }
        }

        NOT_FOUND
    }
}
