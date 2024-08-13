// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  - なんかいい感じにできなかった
    tail, headを常に閉区間にしたかったのにlenの比較の時だけ開区間になってしまう

  何を考えて解いていたか
  - target以上になるまでtailを動かしていく
    target以上になったらどうするか
    target より小さくなるまでhead + 1してみる
    そしたらtail < nums.len()の限りtail + 1する
    head == nums.len()でおわり





  想定ユースケース
  -

  正解してから気づいたこと
  - tailを基準にして、headをwhileで動かすようにすればよかった
*/

pub struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        const NOT_FOUND: i32 = 0;
        const INITIAL_VAL: i32 = i32::MAX;

        let mut len = INITIAL_VAL;
        let mut head = 0;
        let mut tail = 0;
        let mut sum = 0;

        while head < nums.len() {
            while sum < target && tail < nums.len() {
                sum += nums[tail];
                tail += 1
            }
            if tail == nums.len() && sum < target {
                break;
            }
            len = len.min((tail - head) as i32);
            sum -= nums[head];
            head += 1;
        }
        match len == INITIAL_VAL {
            true => NOT_FOUND,
            false => len,
        }
    }
}
