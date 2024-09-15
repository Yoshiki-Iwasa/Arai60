// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - tを一文字ずつ舐めていって、sの文字と一致したらsから消していく
    最後に何も残らなければOKなのでは？

  想定ユースケース
  -

  正解してから気づいたこと
  - passした後、leetcodeの答えをみてGreedyやDPによる解き方があることを知る
  - レーベンシュタイン距離という概念を知る
    https://ja.wikipedia.org/wiki/%E3%83%AC%E3%83%BC%E3%83%99%E3%83%B3%E3%82%B7%E3%83%A5%E3%82%BF%E3%82%A4%E3%83%B3%E8%B7%9D%E9%9B%A2
    アルゴリズム解説: https://mathwords.net/hensyukyori


*/

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut s_chars = s.chars().collect::<Vec<_>>();
        s_chars.reverse();
        t.chars().for_each(|t_c| {
            if s_chars.last().is_some_and(|&s_c| s_c == t_c) {
                s_chars.pop();
            }
        });

        s_chars.is_empty()
    }
}
