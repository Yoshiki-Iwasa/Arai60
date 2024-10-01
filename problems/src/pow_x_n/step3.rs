// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

/*
  時間計算量:O(log |n|)
  空間計算量:O(1)
*/

pub struct Solution;

impl Solution {
    pub fn my_pow(x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return 0.0;
        }

        match n.is_positive() {
            true => Self::pow_loop(x, n as u32),
            false => 1.0 / Self::pow_loop(x, n.unsigned_abs()),
        }
    }

    pub fn pow_loop(mut x: f64, mut n: u32) -> f64 {
        let mut powered = 1.0;

        while n != 0 {
            if n & 1 != 0 {
                powered *= x
            }
            x *= x;
            n >>= 1;
        }

        powered
    }

    pub fn pow_recursive(x: f64, n: u32) -> f64 {
        if n == 0 {
            return 1.0;
        }

        if n % 2 == 0 {
            Self::pow_recursive(x * x, n / 2)
        } else {
            Self::pow_recursive(x * x, n / 2) * x
        }
    }
}
