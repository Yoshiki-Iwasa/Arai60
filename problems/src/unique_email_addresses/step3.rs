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
  空間計算量: O(n)
*/

pub struct Solution;

use std::collections::HashSet;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut forwarding_address_set = HashSet::new();
        for email in emails {
            let Some((local_name, domain)) = email.split_once('@') else {
                eprintln!("Invalid Input. No '@' in the address. address:{}", email);
                return i32::MIN;
            };

            let mut forwarding_local_name = String::new();
            for c in email.chars() {
                match c {
                    '.' => continue,
                    '+' => break,
                    other => forwarding_local_name.push(other),
                }
            }
            forwarding_address_set.insert(format!("{}@{}", forwarding_local_name, domain));
        }
        forwarding_address_set.len() as i32
    }
}
