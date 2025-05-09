/*
面试题 01.04. 回文排列
给定一个字符串，编写一个函数判定其是否为某个回文串的排列之一。

回文串是指正反两个方向都一样的单词或短语。排列是指字母的重新排列。

回文串不一定是字典当中的单词。

示例1：
  输入："tactcoa"
  输出：true（排列有"tacocat"、"atcocta"，等等）
*/
use crate::data::Solution;
impl Solution {
  pub fn can_permute_palindrome(s: String) -> bool {
    let mut a = 0u64;
    let mut b = 0u64;
    for c in s.into_bytes() {
      if c < 64 {
        a ^= 1 << c;
      } else {
        b ^= 1 << (c - 64);
      }
    }
    a.count_ones() + b.count_ones() < 2
  }
}
