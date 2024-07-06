// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - うまいグラフの作成方法
  - もう少し効率のいい探索方法

  何を考えて解いていたか
  - word_listとbegin-endをグラフとして考える
  - 一文字ずつしか変更できないなら、次にどのワードに行けるかがわかる
  - graphの構築が完了したらBFSで最短経路を求めればOK
  - 到達できない場合は0を返す
  - 一文字だけ違うという状況をどう表現するか
  - グラフさえ作れれば勝ったも同然

  想定ユースケース
  - 思いつかず

  正解してから気づいたこと
  - 文字列に前処理をすればよかった。
    一文字ずつしか変化しないのであれば、hit は *it, h*t, ht* と表現できてどれかに該当すれば次に行ける
    文字同士をすべて比べる必要はなかった
  - end_wordにたどり着いたらreturnする実装にすればよかった

*/

pub struct Solution;
use std::collections::{HashMap, VecDeque};

impl Solution {
    fn is_in_sequence(s1: &str, s2: &str) -> bool {
        assert!(s1.len() == s2.len());

        let mut same_char_count = 0;
        for (c1, c2) in s1.chars().zip(s2.chars()) {
            if c1 == c2 {
                same_char_count += 1;
            }
        }

        same_char_count >= s1.len() - 1
    }
    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut graph = HashMap::<&str, Vec<&str>>::new();
        let mut dist = HashMap::<&str, usize>::new();

        let mut all_words = vec![];

        if !word_list.contains(&begin_word) {
            all_words.push(&begin_word);
        }
        all_words.extend(word_list.iter());

        for i in 0..all_words.len() {
            for j in i + 1..all_words.len() {
                let s1 = &all_words[i];
                let s2 = &all_words[j];
                if Self::is_in_sequence(s1, s2) {
                    graph.entry(s1).or_insert(vec![]).push(s2);
                    graph.entry(s2).or_insert(vec![]).push(s1);
                }
            }
        }

        // bfs
        let mut queue = VecDeque::new();
        queue.push_back(begin_word.as_str());
        dist.insert(&begin_word, 0);

        while !queue.is_empty() {
            let word = queue.pop_front().unwrap();

            let current_dist = dist.get(word).unwrap().clone();

            if let Some(next_words) = graph.get(word) {
                for next_word in next_words {
                    if dist.contains_key(next_word) {
                        continue;
                    }
                    dist.insert(&next_word, current_dist + 1);
                    queue.push_back(next_word);
                }
            }
        }

        (*dist.get(end_word.as_str()).unwrap_or(&0) + 1) as i32
    }
}
