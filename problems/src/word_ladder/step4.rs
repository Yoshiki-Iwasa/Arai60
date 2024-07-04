// Step4
// 目的: 指摘対応

// bidirectional bfsがちゃんと両方のqueueに入るようにする
// begin側とend側でコピペするのを避けるために`IsFromBegin`を定義し、queue, visitedを一つにまとめる
pub struct Solution;

use std::collections::{HashMap, HashSet, VecDeque};

impl Solution {
    fn to_intermediate_word(word: &str, change_index: usize) -> String {
        format!("{}*{}", &word[0..change_index], &word[change_index + 1..])
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

        // bidirectional bfs search
        type Step = i32;
        type IsFromBegin = bool;

        let mut queue = VecDeque::<(&str, IsFromBegin, Step)>::new();
        queue.push_back((begin_word.as_str(), true, 1));
        queue.push_back((end_word.as_str(), false, 1));

        let mut visited = HashMap::<(&str, IsFromBegin), Step>::new();
        visited.insert((begin_word.as_str(), true), 1);
        visited.insert((end_word.as_str(), false), 1);

        while !queue.is_empty() {
            let (word, is_from_begin, step) = queue.pop_front().unwrap();
            for change_index in 0..word.len() {
                let intermediate = Self::to_intermediate_word(word, change_index);
                for next_word in intermediate_word_map.get(&intermediate).unwrap_or(&vec![]) {
                    if let Some(other_step) = visited.get(&(*next_word, !is_from_begin)) {
                        return step + other_step;
                    }

                    if !visited.contains_key(&(next_word, is_from_begin)) {
                        visited.insert((&next_word, is_from_begin), step + 1);
                        queue.push_back((next_word, is_from_begin, step + 1));
                    }
                }
            }
        }
        0
    }
}
