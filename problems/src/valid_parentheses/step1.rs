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
  - 入れ子状態を解決するのがミソ
  - 正しいカッコの中には正しいカッコが入っているのでstackとして見ていく

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        // RustではVecをstackとして使いがち
        let mut stack = vec![];

        for c in s.chars() {
            let Some(top) = stack.last() else {
                stack.push(c);
                continue;
            };

            let is_match = matches!((top, c), ('(', ')') | ('{', '}') | ('[', ']'));

            if is_match {
                stack.pop();
            } else {
                stack.push(c)
            }
        }
        stack.is_empty()
    }
}
