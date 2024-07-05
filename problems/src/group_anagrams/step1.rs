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
  - 自分がアナグラムをまとめてねって言われたときのことを考えると
    バラバラだと扱いにくいので、全部の文字を昇順に並べ直したくなる
    オリジナルとソート済みの文字列をセットにして扱って、まとめる作業をしたい

  - 要素を一つずつソートしてキーにすればいいのでは？
    各文字列のソートにO(k logk)
    HashMapに詰めるのでO(n)
    HashMapから要素を抜き出すのは最悪O(n)
    O(nklogk)で解けそう

  - キーにするのを何にするか
    今回来るのは、英数字小文字だけを想定してるので、以下の考慮はやりすぎかも

    候補
    - byte列
    byte列をキーにすると、ソートしたときに困る。[0, 1], [1, 0]が同じキーになってしまうので
    - char列
    charはユニコードポイントを指すので、文字列をうまくソートできそう
    graphem clusterが来たときは大丈夫かな？
    charがユニコードポイント毎に分解してくれるので、キーの不整合はおきなさそう
    String型が来ている時点で、valid-UTF8は保証されているので他のエンコーディング方式を考慮しなくても良い



  正解してから気づいたこと
  - 文字の出現回数を数える方法もあるのか
    確かに、自分がアナグラムまとめをやるなら、文字の出現回数を数えた方が速そう
    ソートは文字列が長くなってきたら結構たいへん
    これだと、2byte以上の文字や、合成文字が出てきたときには対応できないな

  - Invalid Inputを考慮するなら、ちゃんとその処理も入れればよかった




*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map = HashMap::<Vec<char>, Vec<String>>::new();
        strs.into_iter().for_each(|s| {
            let mut chars = s.chars().collect::<Vec<_>>();
            chars.sort();
            anagram_map.entry(chars).or_default().push(s);
        });

        anagram_map.into_values().collect::<Vec<_>>()
    }
}
