/*
20. Valid Parentheses
Easy

Given a string s containing just the characters '(', ')', '{', '}', '[' and ']',
 determine if the input string is valid.

An input string is valid if:
  Open brackets must be closed by the same type of brackets.
  Open brackets must be closed in the correct order.

Example 1:
  Input: s = "()"
  Output: true

Example 2:
  Input: s = "()[]{}"
  Output: true

Example 3:
  Input: s = "(]"
  Output: false

Constraints:
  1 <= s.length <= 104
  s consists of parentheses only '()[]{}'.
*/
use crate::Solution;
impl Solution {
  pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];
    for &c in s.as_bytes() {
      stack.push(match c {
        b'(' => b')', b'[' => b']', b'{' => b'}',
        _ => if stack.pop() == Some(c)
        { continue } else { return false }
      });
    }
    stack.is_empty()
  }
}
