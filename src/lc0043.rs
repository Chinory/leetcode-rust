/*
43. Multiply Strings
Medium

Given two non-negative integers num1 and num2 represented as strings, return the product of num1 and num2, also represented as a string.

Note: You must not use any built-in BigInteger library or convert the inputs to integer directly.

Example 1:
  Input: num1 = "2", num2 = "3"
  Output: "6"

Example 2:
  Input: num1 = "123", num2 = "456"
  Output: "56088"

Constraints:
  1 <= num1.length, num2.length <= 200
  num1 and num2 consist of digits only.
  Both num1 and num2 do not contain any leading zero, except the number 0 itself.
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Multiply Strings.
  - Memory Usage: 2 MB, less than 88.89% of Rust online submissions for Multiply Strings.
   */
  pub fn multiply(num1: String, num2: String) -> String {
    fn sub(x: &mut u8) { *x -= b'0' }
    let mut c1 = num1.into_bytes();
    let mut c2 = num2.into_bytes();
    c1.iter_mut().for_each(sub); c1.reverse();
    c2.iter_mut().for_each(sub); c2.reverse();
    let mut ans = vec![0u16; c1.len() + c2.len()];
    for i in 0..c1.len() {
      for j in 0..c2.len() {
        ans[i+j] += (c1[i] * c2[j]) as u16;
      }
    }
    let mut car = 0u32; // carry
    let mut ans: Vec<u8> = ans.into_iter().map(|n|{
      car += n as u32;
      let ch = (car % 10) as u8 + b'0';
      car /= 10;
      ch
    }).collect();
    loop { match ans.pop() {
      Some(b'0') => {},
      Some(c) => break ans.push(c),
      None => return String::from('0'),
    } }
    ans.reverse();
    unsafe { String::from_utf8_unchecked(ans) }
  }
}
