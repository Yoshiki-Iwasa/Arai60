// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  文字列の長さをM, 文字列数をNとしたとき
  O(N) 双方向幅優先探索の最悪計算量
  O(M) 中間表現生成にかかる計算量
    中間表現のmapと、探索時どちらにも使用する

  時間計算量: O(M^2 * N)

  M個の中間表現にたいし、M個の元の単語を保存する
  探索時の空間計算量がO(N)

  空間計算量: O(M^2 * N)
*/

pub struct Solution;

type Step = i32;

use std::collections::{HashMap, VecDeque};

impl Solution {
    fn to_intermediate_word(word: &str, change_index: usize) -> String {
        format!("{}*{}", &word[0..change_index], &word[change_index + 1..])
    }

    fn visit_nodes<'a>(
        queue: &mut VecDeque<(&'a str, Step)>,
        visited: &mut HashMap<&'a str, Step>,
        other_visited: &HashMap<&str, Step>,
        intermediate_word_map: &'a HashMap<String, Vec<&str>>,
    ) -> Option<Step> {
        for _ in 0..queue.len() {
            let (word, step) = queue.pop_front().unwrap();

            for change_index in 0..word.len() {
                let intermediate = Self::to_intermediate_word(word, change_index);
                for next_word in intermediate_word_map.get(&intermediate).unwrap_or(&vec![]) {
                    if let Some(other_step) = other_visited.get(*next_word) {
                        return Some(step + other_step);
                    }

                    if !visited.contains_key(next_word) {
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
            for change_index in 0..word.len() {
                let intermediate = Self::to_intermediate_word(word, change_index);
                intermediate_word_map
                    .entry(intermediate)
                    .or_insert(vec![])
                    .push(&word)
            }
        }

        // bidirectional bfs

        let mut queue_begin = VecDeque::<(&str, Step)>::new();
        let mut queue_end = VecDeque::<(&str, Step)>::new();
        let mut visited_begin = HashMap::<&str, Step>::new();
        let mut visited_end = HashMap::<&str, Step>::new();

        queue_begin.push_back((&begin_word, 1));
        visited_begin.insert(&begin_word, 1);

        queue_end.push_back((&end_word, 1));
        visited_end.insert(&end_word, 1);

        while !queue_begin.is_empty() && !queue_end.is_empty() {
            if let Some(ans) = if queue_begin.len() <= queue_end.len() {
                Self::visit_nodes(
                    &mut queue_begin,
                    &mut visited_begin,
                    &visited_end,
                    &intermediate_word_map,
                )
            } else {
                Self::visit_nodes(
                    &mut queue_end,
                    &mut visited_end,
                    &visited_begin,
                    &intermediate_word_map,
                )
            } {
                return ans;
            }
        }

        0
    }
}
