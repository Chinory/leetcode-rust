/*
9. Palindrome Number
Easy

Given an integer x, return true if x is palindrome integer.
An integer is a palindrome when it reads the same backward as forward.
For example, 121 is a palindrome while 123 is not.
*/
use crate::data::Solution;
impl Solution {
  pub fn is_palindrome(x: i32) -> bool {
    if x == 0 {
      true
    } else if x < 0 || x % 10 == 0 {
      false
    } else {
      let mut a = x;
      let mut y = 0;
      while a != 0 {
        y = y * 10 + a % 10;
        a /= 10;
      }
      x == y
    }
  }
}
