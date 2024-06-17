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
  - cがopen bracketだったらstack.pushみたいな条件分岐をしてるコードがある
    でもやりたいことは「stackの先頭と、次に来る文字を見比べてペアができたらpop」
    そうでなければpush」
    なのでは？
    だとしたら分岐条件は"ペアができること"だけにしぼっていいのでは？

  -

  改善する時にかんがえたこと
  - cをiteratorで回してもいいかも
  - 正直これはどちらでもいい
  - for_eachにすると可読性下がりそう

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

            let is_match = match (top, c) {
                ('(', ')') => true,
                ('{', '}') => true,
                ('[', ']') => true,
                _ => false,
            };

            if is_match {
                stack.pop();
            } else {
                stack.push(c)
            }
        }
        stack.is_empty()
    }
}
