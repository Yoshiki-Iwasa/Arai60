// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - バックトラックの枝狩り部分がどのくらい効率化できているかの持つ森
  - ソートしないでやる方法があるのだろうか


  何を考えて解いていたか
  - candidatesの中から何回でも選べるのか
    答えの組み合わせは150個以下になるとな。この制約は何

    candidatesをsortすることでbacktrackingの無駄な枝狩りができるか
    // 2,3,6,7, target = 7

    // 2 x
    // 2 2 x
    // 2 2 2 x
    // 2,2,2,2 x : この時点であとは見なくて良い
    // 2,2,3 o : この時点であとは見なくて良い
    // 2,3 x
    // 2,3,3 x : この時点であとは見なくて良い
    // 2,6 x : この時点であとは見なくて良い
    // 3,6 x : この時点であとは見なくて良い
    // 6 x
    // 6,7 x
    // 7 o

    こんな感じで枝狩りしながら勧めていく

    時間計算量
    - まずソートにO(N logN)
    - バックトラックの部分の時間計算量はどう見積もったら良いんだろう...
      入力サイズだけに依存しない
      combinationを作るために必要な最大要素数をTとすると、各数字選ぶか選ばないかがあるからO(2^T)

      最悪: O(N x logN x 2^T)


  想定ユースケース
  -

  正解してから気づいたこと
  -
*/

pub struct Solution;

impl Solution {
    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        candidates.sort();
        let mut combinations = vec![];
        Self::combination_sum_helper(&candidates, vec![], 0, target, &mut combinations);
        combinations
    }

    // backtracking
    fn combination_sum_helper(
        // sorted
        candidates: &[i32],
        mut current_combination: Vec<i32>,
        mut current_sum: i32,
        target: i32,
        combinations: &mut Vec<Vec<i32>>,
    ) {
        if current_sum == target {
            combinations.push(current_combination);
            return;
        }

        for (index, &num) in candidates.iter().enumerate() {
            current_combination.push(num);
            current_sum += num;
            // candidatesはソート済みなので、これ以上は必ずtargetを超えてしまう
            if current_sum > target {
                return;
            }
            Self::combination_sum_helper(
                // 現在の要素以上で考える
                &candidates[index..],
                current_combination.clone(),
                current_sum,
                target,
                combinations,
            );
            current_combination.pop();
            current_sum -= num
        }
    }
}
