/*
73. Set Matrix Zeroes
Medium

Given an m x n integer matrix matrix, if an element is 0, set its entire row and column to 0's.

You must do it in place.

Example 1:
  Input: matrix = [[1,1,1],[1,0,1],[1,1,1]]
  Output: [[1,0,1],[0,0,0],[1,0,1]]

Example 2:
  Input: matrix = [[0,1,2,0],[3,4,5,2],[1,3,1,5]]
  Output: [[0,0,0,0],[0,4,5,0],[0,3,1,0]]

Constraints:
  m == matrix.length
  n == matrix[0].length
  1 <= m, n <= 200
  -231 <= matrix[i][j] <= 231 - 1
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 3 ms, faster than 94.44% of Rust online submissions for Set Matrix Zeroes.
  - Memory Usage: 2.3 MB, less than 80.56% of Rust online submissions for Set Matrix Zeroes.
   */
  pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut first_column_is_zero = false;
    let m = matrix.len();
    let n = matrix[0].len();
    for i in 0..m {
      if matrix[i][0] == 0 {
        first_column_is_zero = true;
      }
      for j in 1..n {
        if matrix[i][j] == 0 {
          matrix[i][0] = 0;
          matrix[0][j] = 0;
        }
      }
    }
    for i in (0..m).rev() {
      for j in 1..n {
        if matrix[i][0] == 0 || matrix[0][j] == 0 {
          matrix[i][j] = 0;
        }
      }
      if first_column_is_zero {
        matrix[i][0] = 0;
      }
    }
  }
}
