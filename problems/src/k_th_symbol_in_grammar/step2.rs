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
  - https://github.com/fhiyo/leetcode/pull/47/files
    step1全然自分と違う解き方してる
    とおもったけど、len() / 2をpowerを使って表現している
    兵頭さんのPR、他の回答者のリンクがまとまっててすごい

    文字列の生成パターンとbitの偶奇性をつかってkのみで解くパターンもあるのか
    これは言われれば理解できるけど、思いつける気がしない

    答え決め打ち解法、なるほど。ループで書くならこれか






  他の想定ユースケース
  -


  改善する時にかんがえたこと
  -
*/

pub struct Solution;

impl Solution {
    // 個人的に素直な解き方
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }

        let row_len = 2_i32.pow((n - 1) as u32);

        if k <= row_len / 2 {
            Self::kth_grammar(n - 1, k)
        } else {
            Self::kth_grammar(n - 1, k - row_len / 2) ^ 1
        }
    }

    // 生成パターンとbitの偶奇性で解く おそらく想定解ではなさそう
    // 0(0) -> 0
    // 1(01) -> 1
    // 2(10) -> 1
    // 3(11) -> 0
    // 4(100) -> 1
    pub fn kth_grammar_1(_n: i32, k: i32) -> i32 {
        // 0-indexed
        ((k - 1).count_ones() % 2) as i32
    }

    // 答えを決め打ちしてk==1まで遡る
    // 個人的にはこれも好き
    pub fn kth_grammar_2(mut n: i32, mut k: i32) -> i32 {
        let mut kth_symbol = 0; // assumption
        while k > 1 {
            let row_len = 2_i32.pow((n - 1) as u32);
            if k > row_len / 2 {
                k -= row_len / 2;
                kth_symbol ^= 1
            }

            n -= 1
        }

        // if k == 0 then kth_simbol == 0
        // else kth_simbol == 1
        kth_symbol
    }
}
