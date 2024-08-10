// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(n)
  空間計算量: 1
*/

pub struct Solution;

impl Solution {
    pub fn num_ways(n: i32, k: i32) -> i32 {
        let n = n as usize;
        // ways_to_paint[i] means the number of ways to paint i-th post
        let mut ways_to_paint = Vec::<i32>::with_capacity(n);

        (0..n).for_each(|i| match i {
            0 => ways_to_paint.push(k),
            1 => ways_to_paint.push(k * k),
            _ => ways_to_paint.push((ways_to_paint[i - 1] + ways_to_paint[i - 2]) * (k - 1)),
        });
        ways_to_paint[n - 1]
    }

    pub fn num_ways_2(n: i32, k: i32) -> i32 {
        let n = n as usize;
        let mut ways_to_paint = Vec::<i32>::with_capacity(n);
        ways_to_paint.push(k);
        ways_to_paint.push(k * k);
        Self::num_ways_helper(n - 1, k, &mut ways_to_paint)
    }

    fn num_ways_helper(n: usize, k: i32, ways_to_paint: &mut Vec<i32>) -> i32 {
        if let Some(ways) = ways_to_paint.get(n) {
            return *ways;
        }
        let ways = (Self::num_ways_helper(n - 1, k, ways_to_paint)
            + Self::num_ways_helper(n - 2, k, ways_to_paint))
            * (k - 1);
        ways_to_paint.push(ways);
        ways
    }
}
