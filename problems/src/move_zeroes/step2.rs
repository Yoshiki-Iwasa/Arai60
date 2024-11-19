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
  - https://github.com/fhiyo/leetcode/pull/54/files#diff-2f8b85074aa38861aa9dd6fbe0c5f1b540a06f8618d7552b4ffd05da21f795d3R18
    あ、別の配列つかって最後に入れ替えるのセーフなのか...
    nums以外の配列を定義しちゃいけないのかと思ってしまった

  - https://github.com/fhiyo/leetcode/pull/54/files#diff-2f8b85074aa38861aa9dd6fbe0c5f1b540a06f8618d7552b4ffd05da21f795d3R41
    last_nozero_indexを持っておいてそこと入れ替えるやつ
    直感的ではなかったけどなるほど
    これは再帰的にもかけそう

  - https://github.com/nittoco/leetcode/pull/18/files
    last_nozero_indexという命名には議論の余地あり
    まあでもコメントで書いておけばいいのではなかろうか

  - https://github.com/goto-untrapped/Arai60/pull/25

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  -
*/

pub struct Solution;

impl Solution {
    // last_nonzero_indexを回すやつ
    pub fn move_zeroes_1(nums: &mut [i32]) {
        let mut last_nonzero_index = 0; // last_nonzero_indexより前の数字はすべてnonzero

        (0..nums.len()).for_each(|index| {
            if nums[index] != 0 {
                nums.swap(index, last_nonzero_index);
                last_nonzero_index += 1;
            }
        })
    }

    // 計算量わるいけど、再帰で短く書ける
    // 000123 みたいなときにO(n^2)かかる
    pub fn move_zeroes_2(nums: &mut [i32]) {
        for index in 0..nums.len() {
            if nums[index] != 0 {
                nums.swap(index, 0);
                Self::move_zeroes_2(&mut nums[1..]);
                return;
            }
        }
    }

    // 一番直感的だったのはこれ
    pub fn move_zeroes_3(nums: &mut Vec<i32>) {
        let mut zero_cnt = 0;
        (0..nums.len()).for_each(|i| {
            while nums.get(i).is_some_and(|&n| n == 0) {
                nums.remove(i);
                zero_cnt += 1;
            }
        });
        nums.extend(vec![0; zero_cnt]);
    }
}
