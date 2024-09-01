// Step1
// 目的: 方法を思いつく

// 方法
// 5分考えてわからなかったら答えをみる
// 答えを見て理解したと思ったら全部消して答えを隠して書く
// 5分筆が止まったらもう一回みて全部消す
// 正解したら終わり

/*
  何がわからなかったか
  -

  何を考えて解いていたか
  - open parenthesisを入れて良いときとclose parenthesisを入れていいと来を分類してやれば良さそう

  想定ユースケース
  -

  正解してから気づいたこと
  - parenthesesの長さではなく、num_open_paren, num_close_parenのカウントで比較したほうが良さそう
  - &mut resultを引き回すのではなく、再帰関数の返り値をVec<String>にしたほうが読みやすそう
  - num_open_paren, num_close_parenをカウントダウンしていくほうが、nを引数に入れずに済むが、ifの条件が複雑になるか？
*/

pub struct Solution;

impl Solution {
    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut result = vec![];
        Self::generate_parenthesis_helper(n as usize, 0, 0, &mut String::new(), &mut result);
        result
    }

    fn generate_parenthesis_helper(
        n: usize,
        num_open_paren: usize,
        num_close_paren: usize,
        parentheses: &mut String,
        result: &mut Vec<String>,
    ) {
        if parentheses.len() == 2 * n {
            result.push(parentheses.to_string());
            return;
        }

        // open parenを入れて良い条件
        if num_open_paren < n {
            parentheses.push('(');
            Self::generate_parenthesis_helper(
                n,
                num_open_paren + 1,
                num_close_paren,
                parentheses,
                result,
            );
            parentheses.pop();
        }

        // close parenを入れて良い条件
        if num_close_paren < n && num_close_paren < num_open_paren {
            parentheses.push(')');
            Self::generate_parenthesis_helper(
                n,
                num_open_paren,
                num_close_paren + 1,
                parentheses,
                result,
            );
            parentheses.pop();
        }
    }
}
