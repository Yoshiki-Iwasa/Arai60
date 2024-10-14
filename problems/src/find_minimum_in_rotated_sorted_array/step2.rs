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
  - https://github.com/sakupan102/arai60-practice/pull/43#discussion_r1650127380
    rotatedの場合分けがないパターンも書けるのか
    挙動を精査したらOKなんだけど、どういう発想でこうなったのかわからない
    > 右端より大きい値で一番右側にあるものを探すと考えることもできる
    結果的にrotatedにかかわらず機能するけど、なんかしっくりこない...
    なぜだ..

    自分の二分探索はleft, rightを動かして探索範囲を絞っていくイメージ
    - ほしい答えの特性から、leftを返すかrightを返すか考える（たいていleft）
    - loop不変条件を考える（right - left > 1 なのか right > leftなのかなど状況によって変える）
    - midとleft, right, targetの関係を条件式で表現して動かしていく
    条件式のなかにnums.lengthがでてくると"探索範囲どうなってるんだっけ？"ってなってた



  - https://github.com/goto-untrapped/Arai60/pull/24/files
    こっちも配列の右端を見てる




  他の想定ユースケース
  -


  改善する時にかんがえたこと
  - rotatedを判定しないパターンでも書いてみる
    気持ち的には、left, rightが重なるまで二分探索をする
    最終的にleftを返すことにして、leftの動かし方を見てく
    mid > 右端ならまだ動かさないといけないのでleft+1しておく
    そうでなければrightをmidまで持ってきて探索範囲を狭める感じ


*/

pub struct Solution;

impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        assert!(!nums.is_empty());
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            let mid = left + (right - left) / 2;

            if nums[mid] > nums[nums.len() - 1] {
                left = mid + 1
            } else {
                right = mid
            }
        }
        nums[left]
    }
}
