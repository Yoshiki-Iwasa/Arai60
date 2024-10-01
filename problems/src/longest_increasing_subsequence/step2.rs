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
  - https://github.com/fhiyo/leetcode/pull/32/files
    セグ木、わからん
  - https://github.com/SuperHotDogCat/coding-interview/pull/28/files
    方針は同じぽい
  - https://github.com/Exzrgs/LeetCode/pull/18/files
    入力の制限をマジックナンバーにするの、漠然とした違和感からやってなかったけど
    意味不明の挙動を招かないようにするというやつしっくりきた

    自分も含め、全体的に変数名の付け方に困ってるイメージ
    実装はそんなに変わらんかなー



  改善する時にかんがえたこと
  - 練習を兼ねてbinary_searchを書いた
    Vecに実装されているpartition_pointを使うのもあり
    内部でbinary_searchを行っておる
*/

pub struct Solution;

impl Solution {
    pub fn length_of_lis(nums: Vec<i32>) -> i32 {
        // longest increasing subsequence
        let mut lis = vec![];

        nums.into_iter().for_each(|n| {
            let insert_pos = lis.partition_point(|num_in_lis| *num_in_lis < n);
            // binary_searchならこう
            // let insert_pos = Self::binary_search(&lis, |num_in_lis| num_in_lis >= *n);

            match insert_pos >= lis.len() {
                true => lis.push(n),
                false => lis[insert_pos] = n,
            }
        });
        lis.len() as i32
    }

    #[allow(unused)]
    fn binary_search<F>(array: &[i32], key: F) -> usize
    where
        F: Fn(i32) -> bool,
    {
        let mut left = 0;
        let mut right = array.len();

        while left < right {
            let mid = left + (right - left) / 2;

            if key(array[mid]) {
                right = mid
            } else {
                left = mid + 1
            }
        }

        left
    }
}
