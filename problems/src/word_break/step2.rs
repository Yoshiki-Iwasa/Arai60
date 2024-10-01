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
  - https://github.com/fhiyo/leetcode/pull/40
  - https://github.com/sakupan102/arai60-practice/pull/40/files

  breakable[i]を1-indexedで、i番目の文字まで分割できるかと捉えればいいのか
  なるほど

  Trieは知らなかったので実装してみる



  他の想定ユースケース
  -


  改善する時にかんがえたこと
  -
*/

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl Solution {
    // DP
    pub fn word_break_1(s: String, word_dict: Vec<String>) -> bool {
        let word_set = HashSet::<String>::from_iter(word_dict);
        let mut breakable = vec![false; s.len() + 1];
        breakable[0] = true;

        (1..=s.len()).for_each(|i| {
            (0..i).for_each(|j| breakable[i] = breakable[j] && word_set.contains(&s[j..i]))
        });
        breakable[s.len()]
    }

    // Trieを使ったBFS
    pub fn word_break_2(s: String, word_dict: Vec<String>) -> bool {
        let mut trie = Trie::new();
        word_dict.into_iter().for_each(|word| trie.insert(word));

        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        queue.push_back(s.as_str());
        while let Some(s) = queue.pop_front() {
            if s.is_empty() {
                return true;
            }
            if seen.contains(s) {
                continue;
            }
            seen.insert(s);

            (0..=s.len()).for_each(|i| {
                if trie.search(&s[..i]) {
                    queue.push_back(&s[i..])
                }
            })
        }
        false
    }
}

#[derive(Clone, Debug, Default)]
pub struct TrieNode {
    children: HashMap<char, TrieNode>,
    is_word: bool,
}

impl TrieNode {
    pub fn new() -> Self {
        Self {
            children: HashMap::new(),
            is_word: false,
        }
    }
}

pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            node = node.children.entry(c).or_default();
        }
        node.is_word = true
    }

    pub fn search(&self, word: &str) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            match node.children.get(&c) {
                Some(child_node) => node = child_node,
                None => return false,
            };
        }
        node.is_word
    }
}

impl Default for Trie {
    fn default() -> Self {
        Self::new()
    }
}
