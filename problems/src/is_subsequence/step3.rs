// Step3
// 目的: 覚えられないのは、なんか素直じゃないはずなので、そこを探し、ゴールに到達する

// 方法
// 時間を測りながらもう一度解く
// 10分以内に一度もエラーを吐かず正解
// これを3回連続でできたら終わり
// レビューを受ける
// 作れないデータ構造があった場合は別途自作すること

// 一番苦手というか、思いつかなかったDPで解いておく
/*
  時間計算量: O(|T| * |S|)
  空間計算量: O(|T| * |S|)
*/

pub struct Solution;

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut match_count_table = vec![vec![0; s.len() + 1]; t.len() + 1];

        let t_chars = t.chars().collect::<Vec<_>>();
        let s_chars = s.chars().collect::<Vec<_>>();

        (1..=t_chars.len()).for_each(|t_i| {
            (1..=s_chars.len()).for_each(|s_i| match t_chars[t_i - 1] == s_chars[s_i - 1] {
                true => match_count_table[t_i][s_i] = match_count_table[t_i - 1][s_i - 1] + 1,
                false => {
                    match_count_table[t_i][s_i] = std::cmp::max(
                        match_count_table[t_i - 1][s_i],
                        match_count_table[t_i][s_i - 1],
                    )
                }
            })
        });

        match_count_table[t.len()][s.len()] == s.len()
    }
}
