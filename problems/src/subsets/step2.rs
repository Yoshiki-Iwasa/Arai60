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
  - https://github.com/fhiyo/leetcode/pull/51/files#diff-32ae6e97704fc6ebe760617c9b69078b525330c93c332d2e1bac0ce35dc11f4bR7
    あるindexを加えるか加えないかを利用した再起
    yieldになれないので読むのがちょっと大変
    一番素直なのかもしれない。先に思いつくべきはこっちだったか？

  -


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  -
*/

pub struct Solution;

impl Solution {
    // あるindexの値を選ぶか選ばないかで再起させる
    pub fn subsets_1(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::subsets_1_helper(&nums, vec![], 0, &mut result);
        result
    }
    fn subsets_1_helper(nums: &[i32], subset: Vec<i32>, index: usize, result: &mut Vec<Vec<i32>>) {
        if index == nums.len() {
            result.push(subset);
            return;
        }

        // 再帰的に次の要素を含めないパスを探索
        Self::subsets_1_helper(nums, subset.clone(), index + 1, result);
        // 再帰的に次の要素を含めるパスを探索
        let mut new_subset = subset.clone();
        new_subset.push(nums[index]);
        Self::subsets_1_helper(nums, new_subset, index + 1, result);
    }

    // backtracing
    pub fn subsets_3(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::subsets_3_helper(&nums, vec![], 0, &mut result);
        result
    }

    fn subsets_3_helper(
        nums: &[i32],
        mut subset: Vec<i32>,
        start: usize,
        result: &mut Vec<Vec<i32>>,
    ) {
        result.push(subset.clone());
        if start == nums.len() {
            return;
        }
        // 部分集合(subset)に対してある数を入れるパターンといれないパターンで分岐させる
        (start..nums.len()).for_each(|i| {
            subset.push(nums[i]);
            Self::subsets_3_helper(nums, subset.clone(), i + 1, result);
            subset.pop();
        })
    }

    // 各数字について、既存の配列にいれてすべてのパターンを網羅する
    pub fn subsets_4(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result: Vec<Vec<i32>> = vec![];
        result.push(vec![]);
        nums.iter().for_each(|&n| {
            let mut new_subsets = vec![];
            result.iter().for_each(|existing_subset| {
                let mut cloned = existing_subset.clone();
                cloned.push(n);
                new_subsets.push(cloned);
            });
            result.extend(new_subsets);
        });
        result
    }

    // dfs
    // 樹形図を追っていく感じで書いてみる
    pub fn subsets_5(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut stack = vec![];
        stack.push((vec![], 0));

        let mut result = vec![];
        while let Some((subset, start)) = stack.pop() {
            result.push(subset.clone());
            if start == nums.len() {
                continue;
            }
            (start..nums.len()).for_each(|i| {
                let mut new_subset = subset.clone();
                new_subset.push(nums[i]);
                stack.push((new_subset, i + 1))
            });
        }

        result
    }
}
