/*
10. Regular Expression Matching
Hard

Given an input string s and a pattern p, implement regular expression matching with support for '.' and '*' where:

'.' Matches any single character.
'*' Matches zero or more of the preceding element.
The matching should cover the entire input string (not partial).

Example 1:
  Input: s = "aa", p = "a"
  Output: false
  Explanation: "a" does not match the entire string "aa".

Example 2:
  Input: s = "aa", p = "a*"
  Output: true
  Explanation: '*' means zero or more of the preceding element, 'a'. Therefore, by repeating 'a' once, it becomes "aa".

Example 3:
  Input: s = "ab", p = ".*"
  Output: true
  Explanation: ".*" means "zero or more (*) of any character (.)".

Constraints:
  1 <= s.length <= 20
  1 <= p.length <= 30
  s contains only lowercase English letters.
  p contains only lowercase English letters, '.', and '*'.
  It is guaranteed for each appearance of the character '*', there will be a previous valid character to match.
*/
use crate::data::Solution;
impl Solution {
  pub fn is_match(s: String, p: String) -> bool {
    let s = s.into_bytes();
    let p = p.into_bytes();
    let eq = |i, j| -> bool {
      i != 0 && (p[j - 1] == '.' as u8 || p[j - 1] == s[i - 1])
    };
    let mut dp = vec![vec![false;p.len()+1];s.len()+1];
    dp[0][0] = true;
    for i in 0..=s.len() {
      for j in 1..=p.len() {
        dp[i][j] = if p[j - 1] == '*' as u8 {
          eq(i, j - 1) && dp[i - 1][j] || dp[i][j - 2]
        } else {
          eq(i, j) && dp[i - 1][j - 1]
        }
      }
    }
    dp[s.len()][p.len()]
  }
}
