// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(log n)
  空間計算量: O(1)
*/

pub struct Solution;

impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();

        while left < right {
            let mid = left + (right - left) / 2;

            match nums[mid].cmp(&target) {
                std::cmp::Ordering::Less => left = mid + 1,
                std::cmp::Ordering::Greater => right = mid,
                std::cmp::Ordering::Equal => {
                    return mid as i32;
                }
            }
        }
        left as i32
    }

    // 標準ライブラリバージョン
    pub fn search_insert_2(nums: Vec<i32>, target: i32) -> i32 {
        nums.binary_search(&target)
            .unwrap_or_else(|insert_position| insert_position) as i32
    }
}
