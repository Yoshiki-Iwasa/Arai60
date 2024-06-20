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
  - ソートして二分探索ってあったけど、ホント？
    二分探索で一個ずつ見てくのか
    ソートでO(nlogn), 探索でO(nlogn)か
    インデックスの保持もあるから大変そう

  改善する時にかんがえたこと
  - 変数名。complement
  - ペアができなかった場合について
    空の配列を返しつつ、エラーをログに残すようにした
        関数のインターフェースとして`Vec`が指定されている
        この関数の呼び出し元は`Vec`を期待しているので、不正な入力に対してもそれを返してあげる
        加えて、あとから異常の解析ができるようにエラーログを残しておく
        普通は、tracing crateを使うが、今回それないのでeprintln!()
        https://docs.rs/tracing

    panicという選択肢もあり
        panicさせるのが正解のときは、不正な入力が即時終了すべき異常として判断する時

    どちらにしても基本はResultかOptionを使って安全にエラー処理したい
*/

pub struct Solution;
use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut compliment_map = HashMap::<i32, usize>::new();
        for (index, n) in nums.iter().enumerate() {
            let compliment = target - *n;
            match compliment_map.get(&compliment) {
                Some(compliment_index) => return vec![index as i32, *compliment_index as i32],
                None => {
                    compliment_map.insert(*n, index);
                }
            }
        }
        eprintln!(
            "No pairs can make the target. nums: {:?}, target: {:?}",
            nums, target
        );
        vec![]
    }
}
