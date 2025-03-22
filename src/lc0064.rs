/*
64. Minimum Path Sum
Medium

Given a m x n grid filled with non-negative numbers, find a path from top left to bottom right, which minimizes the sum of all numbers along its path.

Note: You can only move either down or right at any point in time.

Example 1:
  Input: grid = [[1,3,1],[1,5,1],[4,2,1]]
  Output: 7
  Explanation: Because the path 1 → 3 → 1 → 1 → 1 minimizes the sum.

Example 2:
  Input: grid = [[1,2,3],[4,5,6]]
  Output: 12

Constraints:
  m == grid.length
  n == grid[i].length
  1 <= m, n <= 200
  0 <= grid[i][j] <= 100
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 2 ms, faster than 77.42% of Rust online submissions for Minimum Path Sum.
  - Memory Usage: 2.4 MB, less than 87.10% of Rust online submissions for Minimum Path Sum.
   */
  pub fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    // dp[i][j] = grid[i][j] + min(dp[i-1][j], dp[i][j-1])
    let mut rows = grid.into_iter();
    let mut left = 0;
    let mut dp = rows.next().unwrap();
    for x in &mut dp {
      left += *x;
      *x = left;
    }
    for row in rows {
      let mut it = row.into_iter();
      let mut left = i32::MAX;
      for min in &mut dp {
        left = left.min(*min) + it.next().unwrap();
        *min = left;
      }
    }
    *dp.last().unwrap()
  }
}

#[test]
fn test() {
  Solution::min_path_sum(vec![vec![1,3,1],vec![1,5,1],vec![4,2,1]]);
}
