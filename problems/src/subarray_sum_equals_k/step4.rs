// Step4
// 目的: 指摘を修正したバージョンを染み込ませる

/*
  時間計算量: O(n)
  空間計算量: O(n)
*/

use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
        let mut cumulative_sum_to_count = HashMap::<i32, u32>::with_capacity(nums.len() + 1);

        cumulative_sum_to_count.insert(0, 1);

        let mut sum = 0;
        let mut subarray_count = 0;
        for n in nums {
            sum += n;
            let complement = sum - k;
            if let Some(cnt) = cumulative_sum_to_count.get(&complement) {
                subarray_count += cnt;
            };
            *cumulative_sum_to_count.entry(sum).or_insert(0) += 1;
        }

        subarray_count as i32
    }

    // 累積和を愚直にやる方法
    pub fn subarray_sum_2(nums: Vec<i32>, k: i32) -> i32 {
        let mut cumulative_sum = Vec::<i32>::with_capacity(nums.len() + 1);
        cumulative_sum.push(0);

        for (index, n) in nums.iter().enumerate() {
            cumulative_sum.push(cumulative_sum[index] + n);
        }

        let mut count = 0;

        for start in 0..nums.len() {
            for end in start + 1..=nums.len() {
                if cumulative_sum[end] - cumulative_sum[start] == k {
                    count += 1;
                }
            }
        }

        count
    }
}
