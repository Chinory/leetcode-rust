/*
5. Longest Palindromic Substring
Medium

Given a string s, return the longest palindromic substring in s.

Example 1:
  Input: s = "babad"
  Output: "bab"
  Explanation: "aba" is also a valid answer.

Example 2:
  Input: s = "cbbd"
  Output: "bb"

Constraints:
  1 <= s.length <= 1000
  s consist of only digits and English letters.
*/
use crate::data::Solution;
impl Solution {
  /*
    Runtime: 3 ms, faster than 94.80% of Rust online submissions for Longest Palindromic Substring.
    Memory Usage: 2.1 MB, less than 89.96% of Rust online submissions for Longest Palindromic Substring.
  */
  pub fn longest_palindrome(s: String) -> String {
    fn expand<T: std::cmp::Eq>(l: &[T], r: &[T]) -> usize {
      l.iter().rev().zip(r.iter()).take_while(|(l,r)| *l == *r).count()
    }
    let v = s.as_bytes();
    let mut it = v.iter();
    let a = match it.next() { Some(&c)=>c, None=> return s };
    let b = match it.next() { Some(&c)=>c, None=> return s };
    let mut n = if a == b { 2 } else { 1 };
    let mut r = 0..n;
    for i in 2..v.len() {
      let e = expand(&v[..i], &v[i..]);
      if n < e << 1 { n = e << 1; r = i-e..i+e; }
      let e = expand(&v[..i-1], &v[i..]);
      if n < e << 1 | 1 { n = e << 1 | 1; r = i-1-e..i+e; }
    }
    unsafe { String::from_utf8_unchecked(v[r].to_vec()) }
  }
}
