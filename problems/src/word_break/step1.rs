// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - 一応DPの問題だったっぽいけど、DPでの解き方がわからなかった


  何を考えて解いていたか
  - 文字列の長さは300
    dictionaryの要素数は1000
    要素の文字数は20
    300 * 1000
    まず、brute forceの場合、20 * 1000 * (300/20)で 最悪 3 * 10 ^5だからまあ間に合いそう
    先頭から順に消していけばいいと思ったが、それだとダメそう
    word dictから作れる組み合わせを考えないといけない
    これを手作業でやるとしたらどうやるだろうか

    合うwordを見つけたら、次の人に辞書と残りの文字列を渡す
    これを全wordに対して行って、残り文字が消えたら成功

    メモ化再帰をしないで実行したらWA食らってしまった
    メモを追加してSuccess


  想定ユースケース
  -

  正解してから気づいたこと
  - 文字列中の文字をnodeとして捉えて、幅優先探索で最後の文字までたどり着けるかやる方法
  -
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        Self::word_break_helper(s.as_str(), &word_dict, &mut HashMap::new())
    }

    fn word_break_helper<'a>(
        target: &'a str,
        word_dict: &[String],
        memo: &mut HashMap<&'a str, bool>,
    ) -> bool {
        if let Some(result) = memo.get(target) {
            return *result;
        }
        if target.is_empty() {
            return true;
        }

        for word in word_dict.iter() {
            if target.starts_with(word) {
                match Self::word_break_helper(&target[word.len()..], word_dict, memo) {
                    true => {
                        memo.insert(target, true);
                        return true;
                    }
                    false => continue,
                }
            }
        }
        memo.insert(target, false);
        false
    }
}
