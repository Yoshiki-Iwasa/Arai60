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
  - https://github.com/fhiyo/leetcode/pull/53/files#diff-9ca14f2eb9507a65f01cd60a9947537f09b5daeede1ef28ee1d790d8d76adb56R22
    '(', ')'の使用可能数をカウントダウンしていく方法のほうが引数が減りそう

  - https://github.com/fhiyo/leetcode/pull/53/files#diff-9ca14f2eb9507a65f01cd60a9947537f09b5daeede1ef28ee1d790d8d76adb56R40
    管理する対象を'('中心に考える方法。本質的にはStep1の解法と変わらない

  - https://discord.com/channels/1084280443945353267/1218823830743547914/1231546400714788864
    カタラン数って何？
    https://stchopin.hatenablog.com/entry/2023/06/18/201048
    この説明わかりやすいですねえ。
    > 他に、開き+閉じ*(0~ここまでの開きの数までのどれか)までを一つの仕事とみるという仕事の分担方法もあるように思います。
    https://github.com/ryoooooory/LeetCode/pull/6/files#diff-0c41c9ee9ae0791aef4377ca71aa7fecf5b6047b52862045b5a302a7f99e3b37
    これは選択肢として出てこなかった

  - https://github.com/rossy0213/leetcode/pull/27#pullrequestreview-2205469470


  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - helper関数の基底条件を'(', ')'の数で表現するように変更した
*/

pub struct Solution;

impl Solution {
    // backtrackの改善版
    // Vec<String>を返すようにし、open / closeの残り使用可能数を引数として渡す
    pub fn generate_parenthesis_1(n: i32) -> Vec<String> {
        Self::create_parenthesis(n as usize, n as usize, &mut String::new())
    }

    fn create_valid_parentheses(
        rest_open_count: usize,
        rest_close_count: usize,
        parentheses: &mut String,
    ) -> Vec<String> {
        if rest_open_count == 0 && rest_close_count == 0 {
            return vec![parentheses.to_string()];
        }

        let mut valid_parentheses = vec![];

        // open parenを入れて良い条件
        if rest_open_count > 0 {
            parentheses.push('(');
            valid_parentheses.extend(Self::create_valid_parentheses(
                rest_open_count - 1,
                rest_close_count,
                parentheses,
            ));
            parentheses.pop();
        }

        // close parenを入れて良い条件
        if rest_close_count > 0 && rest_close_count > rest_open_count {
            parentheses.push(')');
            valid_parentheses.extend(Self::create_parenthesis(
                rest_open_count,
                rest_close_count - 1,
                parentheses,
            ));
            parentheses.pop();
        }
        valid_parentheses
    }

    // iterative
    pub fn generate_parenthesis_2(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut valid_parentheses = vec![];

        let mut stack = vec![];
        // parentheses, num_open_paren, num_close_paren
        stack.push((String::new(), 0, 0));

        while let Some((mut parentheses, num_open_paren, num_close_paren)) = stack.pop() {
            if num_open_paren == n && num_close_paren == n {
                valid_parentheses.push(parentheses);
                continue;
            }

            if num_open_paren < n {
                parentheses.push('(');
                stack.push((parentheses.clone(), num_open_paren + 1, num_close_paren));
                parentheses.pop();
            }

            if num_close_paren < n && num_close_paren < num_open_paren {
                parentheses.push(')');
                stack.push((parentheses.clone(), num_open_paren, num_close_paren + 1));
                parentheses.pop();
            }
        }

        valid_parentheses
    }

    // iterative
    // これまでの開カッコの数まで閉じカッコを足す方法
    // わかりづらいけど、方法の一つとして
    pub fn generate_parenthesis_3(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut valid_parentheses = vec![];

        let mut stack = vec![];
        // parentheses, num_open_paren, num_close_paren
        stack.push((String::new(), 0));

        while let Some((parentheses, num_unclosed_paren)) = stack.pop() {
            if parentheses.len() == 2 * n && num_unclosed_paren == 0 {
                valid_parentheses.push(parentheses);
                continue;
            }

            (0..=num_unclosed_paren + 1).for_each(|num_close_added| {
                if parentheses.len() + 1 + num_close_added > n * 2 {
                    return;
                }
                let mut new_parentheses = parentheses.clone();
                new_parentheses.push('(');
                new_parentheses.push_str(&")".repeat(num_close_added));
                stack.push((new_parentheses, num_unclosed_paren + 1 - num_close_added));
            });
        }

        valid_parentheses
    }
}
