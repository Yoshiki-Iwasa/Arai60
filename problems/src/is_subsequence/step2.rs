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
  - https://github.com/rihib/leetcode/pull/19/files#diff-11e3a36a6ae34b4b9009f23b1ed6f52e205e99f6ea87b22beee2bfbcd4d018ddR7
    シンプル2ポインタによる実装

  - https://github.com/fhiyo/leetcode/pull/55/files#diff-a6c7d5ff748fd033529b0b0a550ed2aa570e18edc3e2c61da5094aec0e23a91eR29
    正規表現はかけない気がする...
    でもなるほどと思った

  - https://github.com/nittoco/leetcode/pull/16/files#diff-4213dd9e9bb2e47ed3241ed4250c36f5193149962dd86c6163c7d6b52b8b2668R50
    他でもみたwhile trueの表現
    while trueできれば避けたいので多分本番では書かないけど、表現としてはあることを頭にいれておく


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - とりあえず、思いつく & 調べた限りの解法で解いておく
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    // step1と同じ。2pointersの亜種
    pub fn is_subsequence_1(s: String, t: String) -> bool {
        let mut s_chars = s.chars().collect::<Vec<_>>();
        s_chars.reverse();
        t.chars().for_each(|t_c| {
            if s_chars.last().is_some_and(|&s_c| s_c == t_c) {
                s_chars.pop();
            }
        });

        s_chars.is_empty()
    }

    // divide and conquer
    // s,tの先頭だけをみて削っていく実装
    pub fn is_subsequence_2(s: String, t: String) -> bool {
        Self::is_subsequence_2_helper(
            s.chars().collect::<Vec<_>>().as_slice(),
            t.chars().collect::<Vec<_>>().as_slice(),
        )
    }

    fn is_subsequence_2_helper(source: &[char], target: &[char]) -> bool {
        let Some(s_first) = source.first() else {
            return true;
        };

        let Some(t_first) = target.first() else {
            return false;
        };

        match s_first == t_first {
            true => Self::is_subsequence_2_helper(&source[1..], &target[1..]),
            false => Self::is_subsequence_2_helper(source, &target[1..]),
        }
    }

    // hashmapを使ってtの文字のindicesを保存する
    // sourceを一文字ずつみていって、tのindexを単調増加させられるかを確認する
    // tが与えられ、sが何個もくるシナリオでもO(|S|)で対応できる
    pub fn is_subsequence_3(s: String, t: String) -> bool {
        let mut char_to_indices = HashMap::<char, Vec<usize>>::new();

        t.char_indices().for_each(|(index, c)| {
            char_to_indices.entry(c).or_default().push(index);
        });

        let mut current_t_index = 0;
        for c in s.chars() {
            let Some(t_indices) = char_to_indices.get(&c) else {
                return false;
            };

            let Some(t_index) = t_indices
                .iter()
                .find(|&&t_index| current_t_index <= t_index)
            else {
                return false;
            };
            current_t_index = t_index + 1;
        }

        true
    }

    // two pointers
    // s_index == s_chars.len() を二回書かねばならず、ちょっときたない
    pub fn is_subsequence_4(s: String, t: String) -> bool {
        let mut s_index = 0;
        let s_chars = s.chars().collect::<Vec<_>>();

        for t_c in t.chars() {
            let Some(&s_c) = s_chars.get(s_index) else {
                return s_index == s_chars.len();
            };

            if s_c == t_c {
                s_index += 1;
            }
        }

        s_index == s_chars.len()
    }

    // DP
    // s,t 一文字ずつ順番にみていって、マッチする文字数を調べておく方法
    pub fn is_subsequence_5(s: String, t: String) -> bool {
        // match_count_table[i][j] means the max number of same ordered characters between t[0;i], s[0;j]
        // first row and column is initialized zero because there is no match between empty string and s,t
        let mut match_count_table = vec![vec![0; s.len() + 1]; t.len() + 1];
        let s_chars = s.chars().collect::<Vec<_>>();
        let t_chars = t.chars().collect::<Vec<_>>();

        (1..=t.len()).for_each(|t_i| {
            // t_i is 1-indexed
            (1..=s.len()).for_each(|s_i| {
                // s_i is 1-indexed
                if t_chars[t_i - 1] == s_chars[s_i - 1] {
                    match_count_table[t_i][s_i] = match_count_table[t_i - 1][s_i - 1] + 1;
                } else {
                    match_count_table[t_i][s_i] = std::cmp::max(
                        match_count_table[t_i - 1][s_i],
                        match_count_table[t_i][s_i - 1],
                    );
                }
            });
        });

        // if s is t's subsequence, the number of same ordered characters between s and t is s.len()
        match_count_table[t.len()][s.len()] == s.len()
    }
}
