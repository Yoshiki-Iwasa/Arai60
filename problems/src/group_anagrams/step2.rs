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
  - メソッドチェーンは各メソッドの処理が単純である場合は有効
    チェーンが深くなる事にワーキングメモリを消費するので、適宜処理は分けるべき

  他の人のコードを読んで考えたこと
  - 文字の出現回数をkeyとする実装
    #で挟んで表現している人がいたけど、これ必要なのか？
    hashmapの性質から、文字の出現回数をカウントする配列をkeyにすればいいのでは
    hash計算の効率次第
    hash計算の効率の肌感はまだない...


  改善する時にかんがえたこと
  - a~z以外で構成された文字列が来た場合にエラーを返せるようにvalidationを行う

  - 文字の出現回数を数える方法でやってみる
  - 関数型の書き方でも書いてみる
    メソッドチェーンで頑張ってもあまり可読性は高くなりそうにない
    不正な入力への対応をしようとしてResult型を導入すると、複雑になる

*/

pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map = HashMap::<[u8; 26], Vec<String>>::with_capacity(strs.len());

        let offset = 'a' as usize;

        for s in strs {
            let mut char_count = [0_u8; 26];
            for char in s.chars() {
                if !char.is_ascii_lowercase() {
                    eprintln!("Invalid string. Contains non-ascii-lowercase. s: {}", s);
                    return vec![];
                }
                char_count[char as usize - offset] += 1;
            }
            anagram_map.entry(char_count).or_default().push(s);
        }

        anagram_map.into_values().collect::<Vec<_>>()
    }

    // 関数でつなげて書いてみたが、あまり向かなかった
    #[allow(clippy::manual_try_fold)]
    pub fn group_anagrams_functional(strs: Vec<String>) -> Vec<Vec<String>> {
        strs.iter()
            .fold(
                Result::<HashMap<[u8; 26], Vec<String>>, String>::Ok(HashMap::<
                    [u8; 26],
                    Vec<String>,
                >::with_capacity(
                    strs.len()
                )),
                |anagram_map, s| {
                    let mut anagram_map = anagram_map?;
                    let char_count = s.chars().fold(Ok([0_u8; 26]), |char_count, char| {
                        let mut char_count = char_count?;
                        if !char.is_ascii_lowercase() {
                            return Err(format!(
                                "Invalid string. Contains non-ascii-lowercase. s: {}",
                                &s,
                            ));
                        }

                        let offset = 'a' as usize;
                        char_count[char as usize - offset] += 1;
                        Ok(char_count)
                    })?;

                    anagram_map
                        .entry(char_count)
                        .or_insert(vec![])
                        .push(s.clone());
                    Ok(anagram_map)
                },
            )
            .map_or_else(
                |err_str| {
                    eprintln!("{}", err_str);
                    vec![]
                },
                |map| map.into_values().collect::<Vec<_>>(),
            )
    }
}
