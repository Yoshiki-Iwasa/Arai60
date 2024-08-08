// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量: O(2N) -> O(N)
  空間計算量: O(1)
*/

// sliding windowだけで解く
pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    const INITIAL_VAL: i32 = i32::MAX;
    const NOT_FOUND: i32 = 0;

    let mut minimum_len = INITIAL_VAL;

    let mut sum = 0;
    let mut head = 0;
    let mut tail = 0;
    while tail < nums.len() {
        sum += nums[tail];
        while sum >= target {
            minimum_len = minimum_len.min((tail - head + 1) as i32);
            sum -= nums[head];
            head += 1
        }
        tail += 1
    }

    match minimum_len == INITIAL_VAL {
        true => NOT_FOUND,
        false => minimum_len,
    }
}

// 累積和を使ってとく方
pub fn min_sub_array_len_2(target: i32, nums: Vec<i32>) -> i32 {
    const INITIAL_VAL: i32 = i32::MAX;
    const NOT_FOUND: i32 = 0;
    let mut minimum_len = INITIAL_VAL;

    let mut prefix_sum = vec![0];
    for i in 0..nums.len() {
        prefix_sum.push(prefix_sum[i] + nums[i])
    }

    let mut left = 0;
    let mut right = 1;

    while right <= nums.len() {
        while prefix_sum[right] - prefix_sum[left] >= target {
            minimum_len = minimum_len.min((right - left) as i32);
            left += 1
        }
        right += 1
    }

    match minimum_len == INITIAL_VAL {
        true => NOT_FOUND,
        false => minimum_len,
    }
}
