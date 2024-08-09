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
  - 標準ライブラリを使えばいいのでは？

  他の人のコードを読んで考えたこと
  -

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        // binary_search()は見つからなかったらその要素が入るべきindexをErr()で返してくれる
        // https://doc.rust-lang.org/std/vec/struct.Vec.html#method.binary_search
        nums.binary_search(&target)
            .unwrap_or_else(|insert_position| insert_position) as i32
    }
}
