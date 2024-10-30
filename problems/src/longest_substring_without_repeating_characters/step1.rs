// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - なんかコレジャナイ
    特にhead = head.max()
    headが移動したらhead以下の歴史は無視したいけど、どうしたもんか

  何を考えて解いていたか
  - head,tailを動かしていくことにする
    charの登場indexを覚えておいて、そこにheadを移動させる
    headが後ろに戻ることのないようにhead.max()にしておく

  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

use std::collections::HashMap;
pub struct Solution;

impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
        let mut head = 0_usize;
        let mut tail = 0_usize;

        let chars = s.chars().collect::<Vec<char>>();

        let mut ans = 0;
        let mut char_to_index = HashMap::<char, usize>::new();
        while tail < chars.len() {
            if let Some(duplicated_char_idx) = char_to_index.get(&chars[tail]) {
                // headが戻らないようにする
                head = head.max(duplicated_char_idx + 1);
            }
            char_to_index.insert(chars[tail], tail);
            ans = ans.max(tail - head + 1);
            tail += 1;
        }

        ans as i32
    }
}
