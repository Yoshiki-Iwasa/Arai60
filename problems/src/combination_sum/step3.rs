// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

// stackで枝狩りするバージョンを書いておく
// 再起でも書けるが、できるだけloopを書くくせをつけておく

/*
  Tはtargetを作るために必要な最大要素数とする
  時間計算量: O(N * logN * 2^T)
  空間計算量: O(2^T)
*/

pub struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();

        let mut result = vec![];
        let mut stack = vec![];
        stack.push((vec![], 0, candidates.as_slice()));

        'outer: while let Some((combination, sum, rest_candidate)) = stack.pop() {
            if sum == target {
                result.push(combination);
                continue;
            }

            for (index, &num) in rest_candidate.iter().enumerate() {
                if sum + num > target {
                    continue 'outer;
                }
                let mut new_combination = combination.clone();
                new_combination.push(num);
                stack.push((new_combination, sum + num, &rest_candidate[index..]));
            }
        }

        result
    }
}
