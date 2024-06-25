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
  - どのようなイメージを持って書いているか

  他の人のコードを読んで考えたこと
  -

  他の想定ユースケース
  - 山の獲得標高がKになるような区間の選定（discordで発見）
    YAMAPみたいな山岳サービスのバックエンドで動いているかもしれない
    その場合、メモリの節約よりも実行速度の追求のほうがビジネス上の優先度高そう
    また、クラッシュしないように安全に作ることも必要か



  改善する時にかんがえたこと
  - 累積和の結果をHashMapに保存して、補数を使えばO(n)でいける
  -
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cumulative_sum_count_map = HashMap::<i32, u32>::new();

        let mut sum = 0;
        let mut subarray_count = 0;

        cumulative_sum_count_map.insert(0, 1);
        for n in nums {
            sum += n;
            let compliment = sum - k;
            if let Some(cnt) = cumulative_sum_count_map.get(&compliment) {
                subarray_count += cnt;
            }
            *cumulative_sum_count_map.entry(sum).or_insert(0) += 1;
        }

        subarray_count as i32
    }
}
