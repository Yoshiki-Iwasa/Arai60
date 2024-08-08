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
  - https://github.com/fhiyo/leetcode/pull/48/files
    HashSetを使った解法だとuniqueな部分文字列が入ってるのがわかりやすいなと思った
    こっちでも解く

    意外とStep1の解法は悪くなかったのか
    tailをiterativeに操作するなら、for文で十分だったかも
  - https://github.com/Mike0121/LeetCode/pull/21/files
    似た感じになってる


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - chars.get(n)で安全なアクセスにすることにした
  - tailをforで回してみた

  - ここに来て空行の
*/

use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    // HashMapでindexを記録する
    pub fn length_of_longest_substring_1(s: String) -> i32 {
        let mut head = 0_usize;
        let mut tail = 0_usize;
        let mut ans = 0;
        let mut char_to_index = HashMap::<char, usize>::new();
        let chars = s.chars().collect::<Vec<_>>();

        while let Some(c) = chars.get(tail) {
            if let Some(duplicated_char_idx) = char_to_index.get(c) {
                // headが戻らないようにする
                head = head.max(duplicated_char_idx + 1);
            }
            char_to_index.insert(*c, tail);
            ans = ans.max(tail - head + 1);
            tail += 1
        }

        ans as i32
    }

    // HashSetでunique文字列を記録する
    pub fn length_of_longest_substring_2(s: String) -> i32 {
        let mut ans = 0;
        let chars = s.chars().collect::<Vec<_>>();
        let mut unique_chars = HashSet::<char>::new();
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
