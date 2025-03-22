/*
76. Minimum Window Substring
Hard

Given two strings s and t of lengths m and n respectively, return the minimum window substring of s such that every character in t (including duplicates) is included in the window. If there is no such substring, return the empty string "".

The testcases will be generated such that the answer is unique.

A substring is a contiguous sequence of characters within the string.

Example 1:
  Input: s = "ADOBECODEBANC", t = "ABC"
  Output: "BANC"
  Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.

Example 2:
  Input: s = "a", t = "a"
  Output: "a"
  Explanation: The entire string s is the minimum window.

Example 3:
  Input: s = "a", t = "aa"
  Output: ""
  Explanation: Both 'a's from t must be included in the window.
  Since the largest window of s only has one 'a', return empty string.

Constraints:
  m == s.length
  n == t.length
  1 <= m, n <= 105
  s and t consist of uppercase and lowercase English letters.

Follow up: Could you find an algorithm that runs in O(m + n) time?
*/
use crate::data::Solution;

impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Minimum Window Substring.
  - Memory Usage: 2.3 MB, less than 82.61% of Rust online submissions for Minimum Window Substring.
   */
  pub fn min_window(s: String, t: String) -> String {
    if s.len() < t.len() { return String::new(); }
    let mut dict = Dict::new();
    let mut unused = 0u32;
    for c in t.into_bytes() {
      if let Some(_) = dict.try_push(c) {
        unused += 1;
      }
    }
    let s = s.into_bytes();
    let mut i = 0;
    let mut j = 0;
    let mut ans = 0..usize::MAX;
    while j < s.len() {
      if let Some(left) = dict.get_mut(s[j]) {
        if *left > 0 {
          unused -= 1;
        }
        *left -= 1;
      }
      j += 1;
      while unused == 0 {
        if let Some(right) = dict.get_mut(s[i]) {
          *right += 1;
          if *right > 0 {
            unused += 1;
            let cur = i..j;
            if ans.len() > cur.len() {
              ans = cur;
            }
          }
        }
        i += 1;
      }
    }
    if ans.end == usize::MAX { return String::new(); }
    // for c in &mut s {
    //   if let Some(_) = dict.get_mut(*c) {
    //     *c = c.to_ascii_uppercase();
    //   }
    // }
    unsafe { String::from_utf8_unchecked(s[ans].to_vec()) }
  }
}

const NULL: i32 = i32::MIN;

struct Dict([i32; 52]);

impl Dict {
  fn new() -> Self {
    Self([NULL; 52])
  }
  fn index_of(c: u8) -> usize {
    (match c {
      b'A'..=b'Z' => c - b'A',
      b'a'..=b'z' => c - b'a' + 26,
      _ => 52
    }) as usize
  }
  fn try_push(&mut self, c: u8) -> Option<()> {
    let r = self.0.get_mut(Self::index_of(c))?;
    Some(if *r == NULL { *r = 1 } else { *r += 1 })
  }
  fn get_mut(&mut self, c: u8) -> Option<&mut i32> {
    let r = self.0.get_mut(Self::index_of(c))?;
    if *r == NULL { None } else { Some(r) }
  }
}

use rand::Rng;

fn rand_gen(n: usize) -> String {
  let mut rng = rand::thread_rng();
  let mut s = Vec::with_capacity(n);
  for _ in 0..n {
    s.push(rng.gen_range(b'a'..=b'z'));
  }
  unsafe { String::from_utf8_unchecked(s) }
}

fn rand_sel(s: &String, n: usize) -> String {
  let mut rng = rand::thread_rng();
  let mut s = s.clone().into_bytes();
  for i in 0..n {
    let j = rng.gen_range(i..s.len());
    s.swap(i, j);
  }
  s.resize(n, b' ');
  unsafe { String::from_utf8_unchecked(s) }
}

#[test]
fn test() {
  let s = rand_gen(100);
  println!("{}", s);
  let t = rand_sel(&s, 25);
  println!("{}", t);
  let ans = Solution::min_window(s, t);
  println!("{}", ans);
}
