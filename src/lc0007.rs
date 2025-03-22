/*
7. Reverse Integer
Medium

Given a signed 32-bit integer x, return x with its digits reversed. 
If reversing x causes the value to go outside
 the signed 32-bit integer range [-231, 231 - 1], then return 0.
Assume the environment does not allow you to store 64-bit integers (signed or unsigned).

Example 1:
  Input: x = 123
  Output: 321

Example 2:
  Input: x = -123
  Output: -321

Example 3:
  Input: x = 120
  Output: 21

Constraints:
  -231 <= x <= 231 - 1
*/
use crate::data::Solution;
impl Solution {
  pub fn reverse(mut x: i32) -> i32 {
    let mut y = 0;
    fn next(y: i32, x: i32) -> Option<i32> {
      y.checked_mul(10)?.checked_add(x % 10)
    }
    while x != 0 {
      match next(y, x) {
        Some(r) => y = r,
        None => return 0
      };
      x /= 10;
    }
    y
  }
}
