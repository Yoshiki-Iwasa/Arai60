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
  - 自分がこの作業をやれと言われたら
  - local name と domain nameに分ける
  - local nameにだけ処理を行って重複を弾いていく
    emal addressのRFC
    https://datatracker.ietf.org/doc/html/rfc5322#section-3.4.1


  正解してから気づいたこと
  - 不正な入力に対してどのように対処するかが悩ましい...
  - 以下の実装では、'@'が無い時に限って、エラーを出している
    でも、ここだけ局所的にチェックするのはいかがなものか
    この関数の責務をforwardingする宛先の数を数えることに絞るべき
    emalのvalidationは別の関数か、できれば型で行いたい
  - 下手にlocalとか省略しないで、local_nameにすべきか
*/

use std::collections::HashSet;
pub struct Solution;

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut forwarding_address_set = HashSet::new();
        for email_addr in &emails {
            let Some((local, domain)) = email_addr.split_once('@') else {
                eprintln!("Invalid input. No '@' in the address: {:?}", email_addr);
                return i32::MIN;
            };

            let mut forwarding_local_name = String::new();
            for c in local.chars() {
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
