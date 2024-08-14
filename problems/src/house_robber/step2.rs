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
  - https://github.com/goto-untrapped/Arai60/pull/36#pullrequestreview-2176783639
  - https://github.com/fhiyo/leetcode/pull/36

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - とりえずたくさんの解法で解いてみた
    一番効率のいいrob_3をstep3でやろうかな
    でもrob_3はrob_2ありきで改造してたどりつくと思うのでrob_2も書く

*/

pub struct Solution;

impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        let mut max_amounts = vec![0; nums.len()];

        (0..nums.len()).for_each(|i| match i {
            0 => max_amounts[0] = nums[0],
            1 => max_amounts[1] = nums[1],
            2 => max_amounts[2] = max_amounts[0] + nums[2],
            _ => {
                // except adjacent house
                max_amounts[i] =
                    std::cmp::max(max_amounts[i - 2] + nums[i], max_amounts[i - 3] + nums[i]);
            }
        });
        // return 0 if nums is empty
        max_amounts.into_iter().max().unwrap_or_default()
    }

    // 一番奥の家から考えていく方法
    pub fn rob_2(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let mut max_amounts_onward = vec![0; nums.len() + 1];

        max_amounts_onward[nums.len()] = 0;
        max_amounts_onward[nums.len() - 1] = nums[nums.len() - 1];

        (0..nums.len() - 1).rev().for_each(|i| {
            max_amounts_onward[i] = std::cmp::max(
                max_amounts_onward[i + 1],
                max_amounts_onward[i + 2] + nums[i],
            )
        });
        max_amounts_onward[0]
    }

    // 配列を使わずにrob_2と同じことをする
    // 空間の節約になるが、命名が難しい。
    pub fn rob_3(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut post_sum = 0; // no robbing
        let mut current_sum = nums[nums.len() - 1]; // robbed n-th house

        (0..nums.len() - 1).rev().for_each(|i| {
            let max_sum = std::cmp::max(current_sum, post_sum + nums[i]);

            post_sum = current_sum;
            current_sum = max_sum;
        });
        current_sum
    }

    // メモ化再帰
    pub fn rob_4(nums: Vec<i32>) -> i32 {
        Self::rob_from_onward(0, &nums, &mut vec![-1; nums.len()])
    }

    fn rob_from_onward(house_pos: usize, nums: &[i32], max_amounts_onward: &mut Vec<i32>) -> i32 {
        if house_pos >= nums.len() {
            return 0;
        }

        if max_amounts_onward[house_pos] > -1 {
            return max_amounts_onward[house_pos];
        }

        max_amounts_onward[house_pos] = std::cmp::max(
            Self::rob_from_onward(house_pos + 1, nums, max_amounts_onward),
            Self::rob_from_onward(house_pos + 2, nums, max_amounts_onward) + nums[house_pos],
        );
        max_amounts_onward[house_pos]
    }
}
