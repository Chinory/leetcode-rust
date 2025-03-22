/*
63. Unique Paths II
Medium

You are given an m x n integer array grid. There is a robot initially located at the top-left corner (i.e., grid[0][0]). The robot tries to move to the bottom-right corner (i.e., grid[m-1][n-1]). The robot can only move either down or right at any point in time.

An obstacle and space are marked as 1 or 0 respectively in grid. A path that the robot takes cannot include any square that is an obstacle.

Return the number of possible unique paths that the robot can take to reach the bottom-right corner.

The testcases are generated so that the answer will be less than or equal to 2 * 109.

Example 1:
  Input: obstacleGrid = [[0,0,0],[0,1,0],[0,0,0]]
  Output: 2
  Explanation: There is one obstacle in the middle of the 3x3 grid above.
  There are two ways to reach the bottom-right corner:
  1. Right -> Right -> Down -> Down
  2. Down -> Down -> Right -> Right

Example 2:
  Input: obstacleGrid = [[0,1],[0,0]]
  Output: 1

Constraints:
  m == obstacleGrid.length
  n == obstacleGrid[i].length
  1 <= m, n <= 100
  obstacleGrid[i][j] is 0 or 1.
*/
use crate::data::Solution;

impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Unique Paths II.
  - Memory Usage: 2.1 MB, less than 84.13% of Rust online submissions for Unique Paths II.
   */
  pub fn unique_paths_with_obstacles(matrix: Vec<Vec<i32>>) -> i32 {
    // dp[i][j] = matrix[i][j]==1 ? 0 : dp[i-1][j] + dp[i][j-1]
    let row1 = matrix.first().unwrap();
    if *row1.first().unwrap() != 0 ||
      *matrix.last().unwrap().last().unwrap() != 0 {
      return 0;
    }
    let mut dp = vec![0; row1.len()];
    dp[0] = 1;
    for row in matrix {
      let mut it = row.into_iter();
      let mut left = 0;
      for cnt in &mut dp {
        if it.next().unwrap() != 0 {
          left = 0;
          *cnt = 0;
        } else {
          left += *cnt;
          *cnt = left;
        }
      }
    }
    *dp.last().unwrap()
  }
}
