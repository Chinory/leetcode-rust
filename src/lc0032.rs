/*
32. Longest Valid Parentheses
Hard

Given a string containing just the characters '(' and ')', find the length of the longest valid (well-formed) parentheses substring.

Example 1:
  Input: s = "(()"
  Output: 2
  Explanation: The longest valid parentheses substring is "()".

Example 2:
  Input: s = ")()())"
  Output: 4
  Explanation: The longest valid parentheses substring is "()()".

Example 3:
  Input: s = ""
  Output: 0

Constraints:
  0 <= s.length <= 3 * 104
  s[i] is '(', or ')'.
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Longest Valid Parentheses.
  - Memory Usage: 2 MB, less than 93.05% of Rust online submissions for Longest Valid Parentheses.
  */
  pub fn longest_valid_parentheses(s: String) -> i32 {
    let s = s.into_bytes();
    let mut m = 0;
    let mut n = 0usize;
    let mut i = 0;
    let mut j = 0;
    while j < s.len() {
      let c = s[j];
      j += 1;
      match c {
        b'(' => n += 1,
        b')' => match n {
          0 => i = j,
          1 => { n = 0; m = m.max(j - i) },
          _ => n -= 1
        }, _ => {}
      }
    }
    if n == 0 {
      return m as i32;
    }
    n = 0;
    i = j - 1;
    loop {
      let c = s[i];
      match c {
        b')' => n += 1,
        b'(' => match n {
          0 => j = i,
          1 => { n = 0; m = m.max(j - i) },
          _ => n -= 1,
        }, _ => {}
      }
      if i > 0 { i -= 1 } else { break }
    }
    m as i32
  }
}

#[test]
fn test() {
  let i = Solution::longest_valid_parentheses("()(()".to_owned());
  println!("{:?}",i);
}
