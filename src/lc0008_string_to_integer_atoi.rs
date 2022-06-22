/*
8. String to Integer (atoi)
Medium

Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer (similar to C/C++'s atoi function).

The algorithm for myAtoi(string s) is as follows:

Read in and ignore any leading whitespace.
Check if the next character (if not already at the end of the string) is '-' or '+'. Read this character in if it is either. This determines if the final result is negative or positive respectively. Assume the result is positive if neither is present.
Read in next the characters until the next non-digit character or the end of the input is reached. The rest of the string is ignored.
Convert these digits into an integer (i.e. "123" -> 123, "0032" -> 32). If no digits were read, then the integer is 0. Change the sign as necessary (from step 2).
If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then clamp the integer so that it remains in the range. Specifically, integers less than -231 should be clamped to -231, and integers greater than 231 - 1 should be clamped to 231 - 1.
Return the integer as the final result.
Note:

Only the space character ' ' is considered a whitespace character.
Do not ignore any characters other than the leading whitespace or the rest of the string after the digits.
*/
use crate::data::Solution;
impl Solution {
  pub fn my_atoi(s: String) -> i32 {
    #[inline]
    fn ctoi(c: char) -> i32 { c as i32 - '0' as i32 }
    let mut it = s.chars().into_iter().skip_while(|&c|c==' ');
    let (x, n) = match it.next() {
      Some('+') => (0, false), Some('-') => (0, true),
      Some(c @'0'..='9') => (ctoi(c), false), _ => return 0
    };
    let mut it = it.take_while(|&c|'0'<=c && c<='9');
    if n { it.try_fold(x, |i,c|i.checked_mul(10)?.checked_sub(ctoi(c))).unwrap_or(i32::MIN) }
    else { it.try_fold(x, |i,c|i.checked_mul(10)?.checked_add(ctoi(c))).unwrap_or(i32::MAX) }
  }
}
