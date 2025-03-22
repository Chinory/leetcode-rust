/*
62. Unique Paths
Medium

There is a robot on an m x n grid. The robot is initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m - 1][n - 1]). The robot can only move either down or right at any point in time.

Given the two integers m and n, return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The test cases are generated so that the answer will be less than or equal to 2 * 109.

Constraints:
  1 <= m, n <= 100
*/
use crate::data::Solution;
use core::mem;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Unique Paths.
  - Memory Usage: 2 MB, less than 63.38% of Rust online submissions for Unique Paths.
   */
  pub fn unique_paths(mut m: i32, mut n: i32) -> i32 {
    // dp[i][j] = dp[i-1][j] + dp[i][j-1]
    if m > n { mem::swap(&mut m, &mut n); }
    let mut dp = vec![1; n as usize];
    for row in 1..m {
      for col in 1..n as usize {
        dp[col] += dp[col-1];
      }
    }
    *dp.last().unwrap()
  }
}
