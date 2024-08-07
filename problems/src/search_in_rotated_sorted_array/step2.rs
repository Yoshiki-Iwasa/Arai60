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
  - https://github.com/fhiyo/leetcode/pull/44
    一回の二分探索でやってる
    けど、むずいなあ
    key関数によってtrue,falseの配列に変えていく発想ではないからだと思う

  - https://discord.com/channels/1084280443945353267/1233295449985650688/1239594872697262121
    一発で解いてるけどこれわからんなあ
    keyの関数をうまいこと設計してるんだけど、わからん...
    https://discord.com/channels/1084280443945353267/1233295449985650688/1239446770761596928
    -2 or +2を探せばいいとは（？＿？）


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - key関数をうまいことつくってスマートにやる方法は理解できなかった（？＿？）
    一発の二分探索で配列を左右に切って探索してく方法はわかったのでそれを書いてみる
*/

pub struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right {
            let mid = left + (right - left) / 2;

            if nums[mid] == target {
                return mid as i32;
            }

            if nums[mid] >= nums[left] {
                // left array is sorted

                if nums[left] <= target && target < nums[mid] {
                    right = mid - 1; // target exist in left~mid
                } else {
                    left = mid + 1; // target exist in right-array
                }
            } else {
                // right is sorted

                // assume mid = left
                if nums[mid] < target && target <= nums[right] {
                    left = mid + 1; // target exist in mid~right
                } else {
                    right = mid - 1; // target exist in left-array
                }
            }
        }

        -1
    }
}
