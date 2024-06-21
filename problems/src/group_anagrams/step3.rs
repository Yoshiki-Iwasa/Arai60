// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n * k)
  空間計算量: O(n)
*/

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map = HashMap::<[u8; 26], Vec<String>>::with_capacity(strs.len());

        for s in strs {
            let mut char_count = [0_u8; 26];
            for char in s.chars() {
                if !char.is_ascii_lowercase() {
                    eprintln!(
                        "Invalid string. Contains non-ascii-lowercase charactor. s: {:?}",
                        s
                    );
                    return vec![];
                }
                let offset = 'a' as usize;
                char_count[char as usize - offset] += 1;
            }
            anagram_map.entry(char_count).or_insert(vec![]).push(s);
        }
        anagram_map.into_values().collect::<Vec<_>>()
    }
}
