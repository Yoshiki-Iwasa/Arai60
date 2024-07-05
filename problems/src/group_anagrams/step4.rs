pub struct Solution;

use std::collections::HashMap;
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut anagram_map = HashMap::<[u32; 26], Vec<String>>::with_capacity(strs.len());

        for s in strs {
            let mut char_count = [0_u32; 26];
            for char in s.chars() {
                if !char.is_ascii_lowercase() {
                    eprintln!(
                        "Invalid string. Contains non-ascii-lowercase charactor. s: {:?}",
                        s
                    );
                    return vec![];
                }
                let offset = 'a' as usize;
                char_count[char as usize - offset] += 1;
            }
            anagram_map.entry(char_count).or_default().push(s);
        }
        anagram_map.into_values().collect::<Vec<_>>()
    }
}
