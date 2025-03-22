/*
54. Spiral Matrix
Medium

Example 1:
  Input: matrix = [[1,2,3],[4,5,6],[7,8,9]]
  Output: [1,2,3,6,9,8,7,4,5]

Example 2:
  Input: matrix = [[1,2,3,4],[5,6,7,8],[9,10,11,12]]
  Output: [1,2,3,4,8,12,11,10,9,5,6,7]

Constraints:
  m == matrix.length
  n == matrix[i].length
  1 <= m, n <= 10
  -100 <= matrix[i][j] <= 100
*/
use crate::data::Solution;

impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Spiral Matrix.
  - Memory Usage: 2.1 MB, less than 79.49% of Rust online submissions for Spiral Matrix.
   */
  pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans = Vec::new();
    let (h, w) = match matrix.first() {
      Some(row) => match row.len() {
        0 => return ans,
        n => (matrix.len(), n)
      }, _ => return ans
    };
    ans.reserve(h * w);
    let (mut u, mut d) = (0, h - 1);
    let (mut l, mut r) = (0, w - 1);
    loop {
      ans.extend_from_slice(&matrix[u][l..=r]);
      if u < d { u += 1 } else { break; };
      ans.extend(matrix[u..=d].iter().map(|row| row[r]));
      if l < r { r -= 1 } else { break; };
      ans.extend(matrix[d][l..=r].iter().rev());
      if u < d { d -= 1 } else { break; };
      ans.extend(matrix[u..=d].iter().rev().map(|row| row[l]));
      if l < r { l += 1 } else { break; };
    }
    ans
  }
}
