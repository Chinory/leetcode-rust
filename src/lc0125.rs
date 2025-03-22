/*
125. Valid Palindrome
Easy

A phrase is a palindrome if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.

Given a string s, return true if it is a palindrome, or false otherwise.

Example 1:
  Input: s = "A man, a plan, a canal: Panama"
  Output: true
  Explanation: "amanaplanacanalpanama" is a palindrome.

Example 2:
  Input: s = "race a car"
  Output: false
  Explanation: "raceacar" is not a palindrome.

Example 3:
  Input: s = " "
  Output: true
  Explanation: s is an empty string "" after removing non-alphanumeric characters.
  Since an empty string reads the same forward and backward, it is a palindrome.

Constraints:
  1 <= s.length <= 2 * 105
  s consists only of printable ASCII characters.
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Valid Palindrome.
  - Memory Usage: 2.4 MB, less than 60.22% of Rust online submissions for Valid Palindrome.
   */
  pub fn is_palindrome(s: String) -> bool {
    let mut it = s.into_bytes().into_iter().filter(u8::is_ascii_alphanumeric);
    while let (Some(l),Some(r)) = (it.next(), it.next_back()) {
      if l.to_ascii_lowercase() != r.to_ascii_lowercase() { return false; }
    }
    true
  }
}
