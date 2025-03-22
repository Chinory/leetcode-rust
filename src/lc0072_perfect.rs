/*
72. Edit Distance
Hard

Given two strings word1 and word2, return the minimum number of operations required to convert word1 to word2.

You have the following three operations permitted on a word:
  Insert a character
  Delete a character
  Replace a character

Example 1:
  Input: word1 = "horse", word2 = "ros"
  Output: 3
  Explanation:
  horse -> rorse (replace 'h' with 'r')
  rorse -> rose (remove 'r')
  rose -> ros (remove 'e')

Example 2:
  Input: word1 = "intention", word2 = "execution"
  Output: 5
  Explanation:
  intention -> inention (remove 't')
  inention -> enention (replace 'i' with 'e')
  enention -> exention (replace 'n' with 'x')
  exention -> exection (replace 'n' with 'c')
  exection -> execution (insert 'u')

Constraints:
  0 <= word1.length, word2.length <= 500
  word1 and word2 consist of lowercase English letters.
*/
use crate::data::Solution;

impl Solution {
  /**
  # Max Optimized
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Edit Distance.
  - Memory Usage: 2.1 MB, less than 94.44% of Rust online submissions for Edit Distance.
   */
  pub fn min_distance(word1: String, word2: String) -> i32 {
    let word1 = word1.into_bytes();
    let word2 = word2.into_bytes();
    let m = word1.len();
    let n = word2.len();
    if m == 0 { return n as i32; }
    if n == 0 { return m as i32; }
    let mut dp: Vec<_> = (1..=n as u32).collect();
    for i in 0..m {
      let word1_i = word1[i];
      let mut left_up = i as u32;
      let mut left = left_up + 1;
      for j in 0..n {
        let up = dp[j];
        if word1_i == word2[j] {
          left = left_up;
        } else {
          left = left_up.min(left.min(up)) + 1;
        }
        dp[j] = left;
        left_up = up;
      }
    }
    dp[n - 1] as i32
  }
}

#[test]
fn test() {
  let res = Solution::min_distance(
    "intention".to_string(), "execution".to_string());
  println!("{:?}", res);
}
