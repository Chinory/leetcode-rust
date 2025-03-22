/*
面试题 01.01. 判定字符是否唯一
实现一个算法，确定一个字符串 s 的所有字符是否全都不同。

示例 1：
  输入: s = "leetcode"
  输出: false

示例 2：
  输入: s = "abc"
  输出: true

限制：
  0 <= len(s) <= 100
  s[i]仅包含小写字母
  如果你不使用额外的数据结构，会很加分。
*/
use crate::data::Solution;
impl Solution {
  pub fn is_unique(astr: String) -> bool {
    let s = astr.into_bytes();
    if s.len()>26 { return false; }
    let mut m = 0u32;
    for c in s {
      if 1 << (c-b'0') as u32 & m != 0 {
        return false;
      }
      m |= 1 << (c-b'0') as u32;
    }
    true
  }
}
