/*
97. Interleaving String
Medium

Given strings s1, s2, and s3, find whether s3 is formed by an interleaving of s1 and s2.

An interleaving of two strings s and t is a configuration where they are divided into non-empty substrings such that:

s = s1 + s2 + ... + sn
t = t1 + t2 + ... + tm
|n - m| <= 1
The interleaving is s1 + t1 + s2 + t2 + s3 + t3 + ... or t1 + s1 + t2 + s2 + t3 + s3 + ...
Note: a + b is the concatenation of strings a and b.

Example 1:
  Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbcbcac"
  Output: true

Example 2:
  Input: s1 = "aabcc", s2 = "dbbca", s3 = "aadbbbaccc"
  Output: false

Example 3:
  Input: s1 = "", s2 = "", s3 = ""
  Output: true

Constraints:
  0 <= s1.length, s2.length <= 100
  0 <= s3.length <= 200
  s1, s2, and s3 consist of lowercase English letters.

Follow up: Could you solve it using only O(s2.length) additional memory space?
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Interleaving String.
  - Memory Usage: 2.2 MB, less than 52.17% of Rust online submissions for Interleaving String.
   */
  pub fn is_interleave(s1: String, s2: String, s3: String) -> bool {
    //f(i,j) = ( f(i-1,j) && s1(i-1)==s3(p) ) || ( f(i,j-1) && s2(j-1)==s3(p) )
    let (s1, s2, s3) =
      (s1.into_bytes(), s2.into_bytes(), s3.into_bytes());
    if s1.len() + s2.len() != s3.len() {
      return false;
    }
    let mut f = vec![vec![false; s2.len()+1]; s1.len()+1];
    f[0][0] = true;
    for i in 0..f.len() {
      for j in 0..f[i].len() {
        let p = i + j - 1;
        if i > 0 {
          f[i][j] |= f[i-1][j] && s1[i - 1] == s3[p];
        }
        if j > 0 {
          f[i][j] |= f[i][j-1] && s2[j - 1] == s3[p];
        }
      }
    }
    f[s1.len()][s2.len()]
  }
}
