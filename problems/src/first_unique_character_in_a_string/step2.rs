// Step2
// 目的: 自然な書き方を考えて整理する

// 方法
// Step1のコードを読みやすくしてみる
// 他の人のコードを2つは読んでみること
// 正解したら終わり

// 以下をメモに残すこと
// 講師陣はどのようなコメントを残すだろうか？
// 他の人のコードを読んで考えたこと
// 改善する時にかんがえたこと

/*
  講師陣はどのようなコメントを残すだろうか？
  -

  他の人のコードを読んで考えたこと
  - シンプルに出現回数を数えるpassと最初にユニークなやつを探すpassで分ければよかったか
  - 順序つきmapによる解法もあるのか
  - indexを保持しないほうがスッキリするんだな
  - なるほど。histogramという命名はいいなあ

  他の想定ユースケース
  - この関数の想定ユースケースが思いつかず...


  改善する時にかんがえたこと
  - indexを保持するとコードが複雑になるので、保持しない
  - char_histogramを用意して認知コストを下げる
  - 関数のinterfaceについて
    `first_uniq_char`では範囲外アクセスの懸念があるので事前に入力をチェックしている
    もし不正入力があった場合は`INVALID_INPUT`を返すように独自に決めた

    `first_uniq_char_hash_map`では範囲外アクセスは発生しないので事前チェックの必要なし
    しかし、合成文字など複数のUnicode pointを持った入力に対して間違ったindexを返す可能性があるので同様にチェックしている

*/

use std::collections::HashMap;

pub struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        const NOT_FOUND: i32 = -1;
        const INVALID_INPUT: i32 = -2;
        const NUMBER_OF_ALPHABETS: usize = 26;

        if !s.chars().any(|c| c.is_ascii_lowercase()) {
            return INVALID_INPUT;
        }
        let mut char_histogram = [0_u32; NUMBER_OF_ALPHABETS];

        let offset = 'a' as usize;
        for c in s.chars() {
            char_histogram[c as usize - offset] += 1;
        }

        for (index, c) in s.chars().enumerate() {
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

        let mut char_count_map = HashMap::<char, u32>::with_capacity(NUMBER_OF_ALPHABETS);

        for c in s.chars() {
            if !c.is_ascii_lowercase() {
                return INVALID_INPUT;
            }
            *char_count_map.entry(c).or_insert(0) += 1;
        }

        for (index, c) in s.char_indices() {
            if char_count_map.get(&c).is_some_and(|cnt| *cnt == 1) {
                return index as i32;
            }
        }

        NOT_FOUND
    }
}
