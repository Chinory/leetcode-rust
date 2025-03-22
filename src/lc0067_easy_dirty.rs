/*
67. Add Binary
Easy

Given two binary strings a and b, return their sum as a binary string.

Example 1:
  Input: a = "11", b = "1"
  Output: "100"

Example 2:
  Input: a = "1010", b = "1011"
  Output: "10101"

Constraints:
  1 <= a.length, b.length <= 104
  a and b consist only of '0' or '1' characters.
  Each string does not contain leading zeros except for the zero itself.
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Add Binary.
  - Memory Usage: 2 MB, less than 77.89% of Rust online submissions for Add Binary.
   */
  pub fn add_binary(a: String, b: String) -> String {
    let (mut long, mut short) =
      if a.len() > b.len() { (a, b) } else { (b, a) };
    let mut ans = Vec::with_capacity(long.len() + 1);
    let mut carry = false;
    let mut it = long.into_bytes().into_iter().rev();
    for s in short.into_bytes().into_iter().rev() {
      ans.push(match (carry, s, it.next().unwrap()) {
        (true, b'0', b'0') => { carry = false; b'1'},
        (true, b'0', _) | (true, _, b'0') => b'0',
        (true, _, _) => b'1',
        (false, b'0', b'0') => b'0',
        (false, b'0', _) | (false, _, b'0') => b'1',
        (false, _, _) => { carry = true; b'0' }
      });
    }
    if carry {
      for l in &mut it {
        if l == b'0' {
          ans.push(b'1');
          carry = false;
          break;
        } else {
          ans.push(b'0')
        }
      }
    }
    if carry {
      ans.push(b'1');
    } else {
      for l in &mut it {
        ans.push(l);
      }
    }
    ans.reverse();
    unsafe { String::from_utf8_unchecked(ans ) }
  }
}
