/*
58. Length of Last Word
Easy

Given a string s consisting of words and spaces, return the length of the last word in the string.

A word is a maximal substring consisting of non-space characters only.

Constraints:
  1 <= s.length <= 104
  s consists of only English letters and spaces ' '.
  There will be at least one word in s.
*/
use crate::data::Solution;
impl Solution {
  pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().last().unwrap().len() as i32
  }
}
