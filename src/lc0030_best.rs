/*
30. Substring with Concatenation of All Words
Hard

You are given a string s and an array of strings words of the same length. Return all starting indices of substring(s) in s that is a concatenation of each word in words exactly once, in any order, and without any intervening characters.
You can return the answer in any order.

Example 1:
  Input: s = "barfoothefoobarman", words = ["foo","bar"]
  Output: [0,9]
  Explanation: Substrings starting at index 0 and 9 are "barfoo" and "foobar" respectively.
  The output order does not matter, returning [9,0] is fine too.

Example 2:
  Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]
  Output: []

Example 3:
  Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]
  Output: [6,9,12]

Constraints:
  1 <= s.length <= 104
  s consists of lower-case English letters.
  1 <= words.length <= 5000
  1 <= words[i].length <= 30
  words[i] consists of lower-case English letters.
*/
use crate::data::Solution;
use std::collections::HashMap;
impl Solution {
  /**
  - Rusty
  - No improper unwrap()
  - Used assert!()
  - Beautiful Code
  - Max Optimized
  - Runtime: 3 ms, faster than 100.00% of Rust online submissions for Substring with Concatenation of All Words.
  - Memory Usage: 2.5 MB, less than 71.43% of Rust online submissions for Substring with Concatenation of All Words.
  */
  pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let mut ans = Vec::new();
    let n = match words.first() {
      Some(w) => w.len(), _ => return ans
    };
    let s = s.into_bytes();
    let bound = match s.len().checked_sub(n) {
      Some(m) => m + 1, _ => return ans
    };
    let mut unused = words.len();
    let mut map = HashMap::new();
    for w in &words {
      *map.entry(w.as_bytes()).or_default() += 1;
    }
    for mut i in 0..n {
      let mut j = i;
      while j < bound {
        // assert(j < s.len());
        let right = &s[j..][..n];
        match map.get_mut(right) {
          None => {
            while i < j {
              let left = &s[i..][..n];
              i += n;
              *map.get_mut(left).unwrap() += 1;
              unused += 1;
            }
            i += n;
          },
          Some(0u32) => {
            assert!(i < j);
            let mut left = &s[i..][..n];
            i += n;
            if left == right {
              if unused == 0 {
                ans.push(i as i32);
              }
            } else {
              loop {
                *map.get_mut(left).unwrap() += 1;
                unused += 1;
                left = &s[i..][..n];
                i += n;
                if left == right { break; }
                assert!(i < j);
              }
            }
          },
          Some(count) => {
            *count -= 1;
            unused -= 1;
            if unused == 0 {
              ans.push(i as i32);
            }
          }
        }
        j += n;
      }
      while i < j {
        let left = &s[i..][..n];
        i += n;
        *map.get_mut(left).unwrap() += 1;
        unused += 1;
      }
    }
    ans
  }
}
