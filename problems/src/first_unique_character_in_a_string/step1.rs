// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 1-passでできないのかしら...

  何を考えて解いていたか
  - とりあえず、文字列のvalidationから
  - 想定文字かを確認。
    想定文字以外がまざっていたらINVALID_INPUTを返すようにする
  - 文字の出現回数とindexをmapにほぞんして回せばいけるか

  想定ユースケース
  - この関数が使われるケースが思いつかない


  正解してから気づいたこと
  - 素直に文字列を2回走査したほうがスッキリ書けそう
  - 解法部分で実質4-passになってるから非効率だ

*/
use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        const NOT_FOUND: i32 = -1;
        const INVALID_INPUT: i32 = -2;
        if !s.chars().any(|c| c.is_ascii_lowercase()) {
            return INVALID_INPUT;
        }
        let mut char_index_map = HashMap::<char, (usize, usize)>::with_capacity(26);
        for (index, c) in s.chars().enumerate() {
            let (_, cnt) = char_index_map.entry(c).or_insert((index, 0));
            *cnt += 1;
        }
        char_index_map
            .iter()
            .filter_map(
                |(_, (index, cnt))| {
                    if *cnt == 1 {
                        Some(index)
                    } else {
                        None
                    }
                },
            )
            .min()
            .map(|index| *index as i32)
            .unwrap_or(NOT_FOUND)
    }
}
