/*
面试题 01.09. 字符串轮转
字符串轮转。给定两个字符串s1和s2，请编写代码检查s2是否为s1旋转而成（比如，waterbottle是erbottlewat旋转后的字符串）。

示例1:
 输入：s1 = "waterbottle", s2 = "erbottlewat"
 输出：True

示例2:
 输入：s1 = "aa", s2 = "aba"
 输出：False

提示：
  字符串长度在[0, 100000]范围内。

说明:
  你能只调用一次检查子串的方法吗？
*/
use crate::data::Solution;
impl Solution {
  pub fn is_fliped_string(s1: String, s2: String) -> bool {
    s1.len() == s2.len() && s2.repeat(2).contains(&s1)
  }
}
