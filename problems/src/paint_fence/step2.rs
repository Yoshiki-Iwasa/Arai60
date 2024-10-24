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
  - https://github.com/goto-untrapped/Arai60/pull/44/files
    メモ化再帰でも実装してみるか
    メモしないと、O(2^n)かかってしまう。毎回の実行で2つに分岐するから
    LRU cacheを使って書いている。なぜ
    Pythonは@cacheつけると関数の結果をメモしといてくれるのか
    ホエー便利
    https://discord.com/channels/1084280443945353267/1200089668901937312/1205887380788351036
    この文脈か

    https://discord.com/channels/1084280443945353267/1200089668901937312/1206182044292485171
    > これに限らず、たまに使う文法があれば、自力で定義できるようにしておいてください。
    Rustだと、
      - `trait Iterator`
      - `macro_rule! vec`
      - `Vec`, `HashMap`,`HashSet`などか
      Vec, HashMapなどはGenericsとTraitのお陰でかなり高度に抽象化できているので、そこも確認
      Rustの強力な型システムは使いこなせると本当に便利だしとってもカッコいい
      この練習はアルゴリズムとは別にやりたい



  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - if i == 0 をやめてmatchにしてみた
    並列に書けて読みやすそう
*/

pub struct Solution;

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        // ways_to_paint[i] means the number of ways to paint i-th post
        let n = n as usize;
        let mut ways_to_paint = Vec::<i32>::with_capacity(n);

        (0..n).for_each(|i| match i {
            0 => ways_to_paint.push(k),
            1 => ways_to_paint.push(k * k),
            _ => ways_to_paint.push((ways_to_paint[i - 1] + ways_to_paint[i - 2]) * (k - 1)),
        });

        ways_to_paint[n - 1]
    }

    // メモ化再帰
    pub fn num_ways_2(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut ways_to_paint = Vec::with_capacity(n);
        ways_to_paint.push(k);
        ways_to_paint.push(k * k);
        Self::num_ways_helper(n - 1, k, &mut ways_to_paint)
    }

    fn num_ways_helper(n: usize, k: i32, ways_to_paint: &mut Vec<i32>) -> i32 {
        if let Some(ways) = ways_to_paint.get(n) {
            return *ways;
        }
        let ways = (Self::num_ways_helper(n - 1, k, ways_to_paint)
            + Self::num_ways_helper(n - 2, k, ways_to_paint))
            * (k - 1);
        ways_to_paint.push(ways);
        ways
    }
}
