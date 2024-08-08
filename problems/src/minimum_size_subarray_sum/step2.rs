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
  - https://github.com/goto-untrapped/Arai60/pull/40/files
    仮にnum < 0 が許容される場合への考慮をしている

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - 内部のwhile条件 sum >= targetにすることで
    left, rightを閉区間として扱える
    rightをforで回してもいけるが、left, rightを同時に宣言したほうが意図が伝わりやすいかなと思った

  - 累積和を使う回答について
    right <= nums.len()とすることでrightがnumsに対して開区間として振る舞うことを示したかった

  - left -> left
    right -> right にした
    https://github.com/Yoshiki-Iwasa/Arai60/pull/42#discussion_r1708937220


*/

pub struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;
        const INITIAL_VAL: i32 = i32::MAX;

        let mut minimum_len = INITIAL_VAL;
        let mut sum = 0;
        let mut left = 0;
        let mut right = 0;

        while right < nums.len() {
            sum += nums[right];

            while sum >= target {
                minimum_len = minimum_len.min((right - left + 1) as i32);
                sum -= nums[left];
                left += 1
            }

            right += 1
        }

        match minimum_len == INITIAL_VAL {
            true => NOT_FOUND,
            false => minimum_len,
        }
    }

    pub fn min_sub_array_len_2(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;
        const INITIAL_VAL: i32 = i32::MAX;

        let mut minimum_len = INITIAL_VAL;
        let mut prefix_sum = vec![0];

        for i in 0..nums.len() {
            prefix_sum.push(prefix_sum[i] + nums[i])
        }

        // [left, right)
        let mut left = 0;
        let mut right = 1;

        // 累積和を使うと区間[l, r)の総和がSr - Slで求められる
        while right <= nums.len() {
            while prefix_sum[right] - prefix_sum[left] >= target {
                minimum_len = minimum_len.min((right - left) as i32);
                left += 1;
            }

            right += 1
        }

        match minimum_len == INITIAL_VAL {
            true => NOT_FOUND,
            false => minimum_len,
        }
    }
}
