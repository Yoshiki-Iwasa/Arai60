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
  - '.'の処理をtrim_maches()でやるほうが見やすいかもしれない。
  - '+'をで処理してあげる方法もある

  改善する時にかんがえたこと
  - str.trim_maches('.')やstr.split_once('+')も検討した
    しかし、forで一文字ずつ見ていったほうが一回の走査で終わるためこのままにした
  - 入力がStringなのは自由度が高すぎるので、EmailAddresを型で定義して使いたいと思った
    email addressのvalidationはこれだけでかなり大きなテーマになりそうなので割愛
    email



*/

use std::{collections::HashSet, str::FromStr};
pub struct Solution;

#[allow(unused)]
// EmailAddress型の例
pub struct EmailAddres {
    local_name: String,
    domain: String,
}

pub struct EmailAddressValidaitonError;

// こんな感じで、EmailAddress型を定義したい
// let email = user_input::parse<EmailAddres>()?
// みたいな使い勝手にしたい。
impl FromStr for EmailAddres {
    type Err = EmailAddressValidaitonError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let Some((local_name, domain)) = s.split_once('@') else {
            return Err(EmailAddressValidaitonError);
        };

        Self::validate_local_name(local_name)?;
        Self::validate_domain(domain)?;

        Ok(Self {
            local_name: local_name.to_string(),
            domain: domain.to_string(),
        })
    }
}

impl EmailAddres {
    fn validate_local_name(_local_name: &str) -> Result<(), EmailAddressValidaitonError> {
        unimplemented!()
    }

    fn validate_domain(_domain: &str) -> Result<(), EmailAddressValidaitonError> {
        unimplemented!()
    }
}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut forwarding_address_set = HashSet::new();
        for email_addr in &emails {
            let Some((local_name, domain)) = email_addr.split_once('@') else {
                eprintln!("Invalid input. No '@' in the address: {:?}", email_addr);
                return i32::MIN;
            };

            let mut forwarding_local_name = String::new();
            for c in local_name.chars() {
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
