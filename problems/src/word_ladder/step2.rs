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
  - https://github.com/kazukiii/leetcode/pull/21/files
    練習としてダイクストラで解いてるが、重みのないグラフなので実際は普通のBFSでいいか

    一文字ずつ入れ替えて探索時に一文字ずつ入れ替えて該当する文字を探す方法をとっている
    おそらく文字列の下処理をしたほうが早く終る

  - https://github.com/sakupan102/arai60-practice/pull/20/files
    connected_word_graphという命名はわかりやすい

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - グラフ化する時の下処理を工夫する
    あるノードの隣接ノードを抽象化して探索を圧縮する

  - 双方向BFSにして時間・空間短縮する
    通常のBFSだと、ステップが深くなるにつれて指数関数的に空間を使用する可能性がある
    双方向BFSで、ならしだが探索時間が短くできる

*/

pub struct Solution;

type Step = i32;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    fn visit_nodes<'a>(
        queue: &mut VecDeque<(&'a str, Step)>,
        visited: &mut HashMap<&'a str, Step>,
        other_visited: &HashMap<&'a str, Step>,
        intermediate_word_map: &HashMap<String, Vec<&'a str>>,
    ) -> Option<Step> {
        for _ in 0..queue.len() {
            let (word, step) = queue.pop_front().unwrap();
            for i in 0..word.len() {
                let intermediate = format!("{}*{}", &word[0..i], &word[i + 1..]);
                for next_word in intermediate_word_map.get(&intermediate).unwrap_or(&vec![]) {
                    if other_visited.contains_key(*next_word) {
                        return Some(step + other_visited[next_word]);
                    }
                    if !visited.contains_key(*next_word) {
                        visited.insert(&next_word, step + 1);
                        queue.push_back((next_word, step + 1));
                    }
                }
            }
        }

        None
    }

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        if !word_list.contains(&end_word) {
            return 0;
        }

        let mut intermediate_word_map = HashMap::<String, Vec<&str>>::new();

        for word in word_list.iter() {
            for i in 0..word.len() {
                let intermediate = format!("{}*{}", &word[0..i], &word[i + 1..]);
                intermediate_word_map
                    .entry(intermediate.to_string())
                    .or_insert(vec![])
                    .push(&word);
            }
        }

        let mut queue_begin = VecDeque::<(&str, Step)>::new();
        let mut visited_begin = HashMap::<&str, Step>::new();
        let mut queue_end = VecDeque::<(&str, Step)>::new();
        let mut visited_end = HashMap::<&str, Step>::new();

        queue_begin.push_back((begin_word.as_str(), 1));
        visited_begin.insert(&begin_word, 1);
        queue_end.push_back((end_word.as_str(), 1));
        visited_end.insert(&end_word, 1);

        // bidirectional breadth first search
        while !queue_begin.is_empty() && !queue_end.is_empty() {
            // queueの小さい方から探索してメモリ使用量を抑える
            if let Some(steps) = if queue_begin.len() <= queue_end.len() {
                Self::visit_nodes(
                    &mut queue_begin,
                    &mut visited_begin,
                    &mut visited_end,
                    &intermediate_word_map,
                )
            } else {
                Self::visit_nodes(
                    &mut queue_end,
                    &mut visited_end,
                    &mut visited_begin,
                    &intermediate_word_map,
                )
            } {
                return steps;
            }
        }

        0
    }
}
