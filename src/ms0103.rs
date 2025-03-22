/*
面试题 01.03. URL化
URL化。编写一种方法，将字符串中的空格全部替换为%20。假定该字符串尾部有足够的空间存放新增字符，并且知道字符串的“真实”长度。（注：用Java实现的话，请使用字符数组实现，以便直接在数组上操作。）

示例 1：
  输入："Mr John Smith    ", 13
  输出："Mr%20John%20Smith"

示例 2：
  输入："               ", 5
  输出："%20%20%20%20%20"

提示：
  字符串长度在 [0, 500000] 范围内。
*/
use crate::data::Solution;
impl Solution {
  pub fn replace_spaces(s: String, length: i32) -> String {
    let mut dst = s.into_bytes();
    let mut src = Vec::from(&dst[..length as usize]);
    dst.clear();
    for c in src {
      if c == b' ' {
        dst.extend_from_slice(&[b'%',b'2',b'0']);
      } else {
        dst.push(c);
      }
    }
    unsafe{ String::from_utf8_unchecked(dst) }
  }
}
