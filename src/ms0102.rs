/*
面试题 01.02. 判定是否互为字符重排
给定两个字符串 s1 和 s2，请编写一个程序，确定其中一个字符串的字符重新排列后，能否变成另一个字符串。

示例 1：
  输入: s1 = "abc", s2 = "bca"
  输出: true

示例 2：
  输入: s1 = "abc", s2 = "bad"
  输出: false

说明：
  0 <= len(s1) <= 100
  0 <= len(s2) <= 100
*/
use crate::data::Solution;
impl Solution {
  pub fn check_permutation(s1: String, s2: String) -> bool {
    let mut n = [0u8; 256];
    for c in s1.into_bytes() {
      n[c as usize] += 1;
    }
    for c in s2.into_bytes() {
      let r = &mut n[c as usize];
      if *r == 0 { return false; }
      *r -= 1;
    }
    true
  }
}
