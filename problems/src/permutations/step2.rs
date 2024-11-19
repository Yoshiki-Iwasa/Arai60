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
  - https://github.com/fhiyo/leetcode/pull/50/files#diff-76c4644fcedcaf9f8c6b6283fa29fa6b79699ef29285914474e37e05a3dbe5f7R17
    nPrの各要素を列挙する解法
    heapのアルゴリズムに似ている

  - https://github.com/fhiyo/leetcode/pull/50/files#diff-76c4644fcedcaf9f8c6b6283fa29fa6b79699ef29285914474e37e05a3dbe5f7R69
    辞書順にpermutationを生成する方法の応用。これはNext Permutationってやつか
    最初見た時は難しいと思った


  - https://github.com/fhiyo/leetcode/pull/50/files#diff-76c4644fcedcaf9f8c6b6283fa29fa6b79699ef29285914474e37e05a3dbe5f7R164
    順列を入れ替えていく方法

  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - とりあえずStep1で思いつかなかった解法をやってみる
  - バックトラックでStep1を書き直すのもやる
    バックトラックのコツは、帰りがけのとき一ステップ戻る処理を入れること

  - heapのアルゴリズムは、swapの場所を部分順列の大きさの偶奇で分ける理由がわからなかった...
*/

pub struct Solution;

impl Solution {
    // 辞書順に生成する
    pub fn permute_1(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        nums.sort();
        result.push(nums.clone());

        while let Some(instance) = Self::next_permutation(nums) {
            result.push(instance.clone());
            nums = instance;
        }
        result
    }

    fn next_permutation(mut nums: Vec<i32>) -> Option<Vec<i32>> {
        // 末尾から見ていって、初めて昇順になるindex
        let Some(pivot_index) = nums.windows(2).rposition(|window| window[0] < window[1]) else {
            // これがないということは降順になっている
            return None;
        };

        // 降順の部分のうち、pivotより大きい最小の値のindexを得る
        let Some(rightmost_successor_index) = nums.iter().rposition(|n| *n > nums[pivot_index])
        else {
            // pivotが存在する以上、rightmost_successorは少なくともpivotの一つ後ろに存在する
            unreachable!()
        };

        // 昇順部分を少し大きくする
        nums.swap(pivot_index, rightmost_successor_index);

        // 降順部分を反転する
        nums[pivot_index + 1..].reverse();

        Some(nums)
    }

    // 樹形図を再帰的につくっていくイメージ
    // backtracing
    pub fn permute_2(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut instance = vec![];
        let mut permutations = vec![];
        Self::create_permutation(&nums, &mut instance, &mut permutations);
        permutations
    }

    fn create_permutation(nums: &[i32], instance: &mut Vec<i32>, permutations: &mut Vec<Vec<i32>>) {
        if instance.len() == nums.len() {
            permutations.push(instance.clone());
            return;
        }

        nums.iter().for_each(|n| {
            if !instance.contains(n) {
                instance.push(*n);
                Self::create_permutation(nums, instance, permutations);
                instance.pop();
            }
        });
    }

    // swappingアルゴリズムを使った順列生成法
    pub fn permute_3(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        Self::swapping_permutation(&mut nums, 0, &mut result);
        result
    }

    fn swapping_permutation(nums: &mut [i32], swap_index: usize, permutations: &mut Vec<Vec<i32>>) {
        if swap_index == nums.len() {
            permutations.push(nums.to_vec());
            return;
        }

        (swap_index..nums.len()).for_each(|i| {
            nums.swap(swap_index, i);
            Self::swapping_permutation(nums, swap_index + 1, permutations);
            // 元に戻す
            nums.swap(swap_index, i);
        })
    }

    // heapのアルゴリズムを使った順列生成法
    pub fn permute_4(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = vec![];
        let perm_size = nums.len();
        Self::heap_permutation(&mut nums, perm_size, &mut result);
        result
    }

    fn heap_permutation(nums: &mut [i32], perm_size: usize, permutations: &mut Vec<Vec<i32>>) {
        if perm_size == 1 {
            permutations.push(nums.to_vec());
            return;
        }

        (0..perm_size).for_each(|i| {
            Self::heap_permutation(nums, perm_size - 1, permutations);
            if perm_size % 2 == 1 {
                nums.swap(0, perm_size - 1);
            } else {
                nums.swap(i, perm_size - 1);
            }
        });
    }
}
