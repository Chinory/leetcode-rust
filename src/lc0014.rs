/*
14. Longest Common Prefix
Easy

Write a function to find the longest common prefix string amongst an array of strings.
If there is no common prefix, return an empty string "".

Constraints:
  1 <= strs.length <= 200
  0 <= strs[i].length <= 200
  strs[i] consists of only lower-case English letters.
*/
use crate::data::Solution;
use std::slice::Iter;
impl Solution {
  pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut vecit: Vec<Iter<u8>> = strs.iter().map(|s|s.as_bytes().iter()).collect();
    let mut ans = String::new();
    loop {
      let mut itit = vecit.iter_mut();
      let first = match match itit
        .next() { Some(it) => it, None => break }
        .next() { Some(&c) => c, None => break };
      if !itit.all(|it| it.next()
        .map_or(false, |&c|c==first)) { break }
      ans.push(first as char);
    }
    ans
  }
}
