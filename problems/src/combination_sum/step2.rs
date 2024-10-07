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
  - https://github.com/nittoco/leetcode/pull/25/files#diff-f084bff8e4dbd771bf8a202d43b499bc30bffb7c10d4c5ccd2102f021910fd19R117
    stackでも解いておこう
  - https://github.com/fhiyo/leetcode/pull/52/files#diff-28c318778976f05919552531da3ec8c28ff86155cf38e57128246b935d3927fbR125
    backtrackingの方針は同じ
    Rustはクロージャでないと環境変数を使えない。環境変数を使えたらもっと引数減らせるのに
    DPでもできるとな。確かに前の状況つかって次の状況を求めていくやつだからできそう

  - https://github.com/hayashi-ay/leetcode/pull/65/files#diff-f084bff8e4dbd771bf8a202d43b499bc30bffb7c10d4c5ccd2102f021910fd19R67
    再起の中でその数を入れる入れないで分岐するやつ
    このほうがスッキリ書けるか？ただ枝狩りはできないのでそこだけ注意



  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - backtrackingは枝狩りをしないほうが読みやすいはずなので、それで書いてみる
*/

pub struct Solution;

impl Solution {
    // iterative
    // 枝狩りなし
    pub fn combination_sum_1(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];

        // combination, sum, rest candidates
        let mut stack = vec![(vec![], 0, candidates.as_slice())];

        while let Some((mut combination, sum, rest_candidates)) = stack.pop() {
            if sum == target {
                result.push(combination);
                continue;
            }
            if sum > target {
                continue;
            }

            let Some((&num, rest)) = rest_candidates.split_first() else {
                continue;
            };
            // 足さずにそのまま進むやつ
            stack.push((combination.clone(), sum, rest));
            combination.push(num);
            // おかわりするやつ
            stack.push((combination, sum + num, rest_candidates));
        }

        result
    }

    // iterative
    // 枝狩りあり
    pub fn combination_sum_2(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // 枝狩りのためにソートしておく
        candidates.sort();

        // combination, sum, rest candidates
        let mut stack = vec![(vec![], 0, candidates.as_slice())];

        let mut result = vec![];

        'outer: while let Some((combination, sum, rest_candidates)) = stack.pop() {
            if sum == target {
                result.push(combination);
                continue;
            }

            for (i, &n) in rest_candidates.iter().enumerate() {
                // 枝狩り
                if sum + n > target {
                    continue 'outer;
                }
                let mut new_combination = combination.clone();
                new_combination.push(n);
                stack.push((new_combination, sum + n, &rest_candidates[i..]))
            }
        }

        result
    }

    // DP
    pub fn combination_sum_3(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        // キャストが面倒なので先にデータ型変えとく
        let candidates = candidates
            .into_iter()
            .map(|n| n as usize)
            .collect::<Vec<_>>();
        let target = target as usize;

        // combination_table[num] contains the combinations whose sum is num
        let mut combinations_table: Vec<Vec<Vec<i32>>> = vec![vec![]; target + 1];
        combinations_table[0].push(vec![]);

        candidates.iter().for_each(|&num| {
            if num > target {
                return;
            }

            combinations_table[num].push(vec![num as i32]);

            (num + 1..=target).for_each(|greater_num| {
                let combinations_for_complement = combinations_table[greater_num - num].clone();

                combinations_for_complement
                    .into_iter()
                    .for_each(|mut comb| {
                        comb.push(num as i32);
                        combinations_table[greater_num].push(comb)
                    });
            })
        });
        // cloneをしないですむように
        combinations_table.remove(target)
    }

    // backtracking
    // 枝狩りなし
    pub fn combination_sum_4(candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::create_combination(&candidates, vec![], 0, target, &mut result);
        result
    }

    fn create_combination(
        candidates: &[i32],
        mut combination: Vec<i32>,
        sum: i32,
        target: i32,
        result: &mut Vec<Vec<i32>>,
    ) {
        if sum == target {
            result.push(combination);
            return;
        }

        if sum > target {
            return;
        }

        let Some((&num, rest)) = candidates.split_first() else {
            return;
        };
        combination.push(num);
        Self::create_combination(candidates, combination.clone(), sum + num, target, result);
        combination.pop();
        Self::create_combination(rest, combination, sum, target, result);
    }
}
