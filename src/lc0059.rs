/*
59. Spiral Matrix II
Medium

Given a positive integer n, generate an n x n matrix filled with elements from 1 to n2 in spiral order.

Example 1:
  Input: n = 3
  Output: [[1,2,3],[8,9,4],[7,6,5]]

Example 2:
  Input: n = 1
  Output: [[1]]

Constraints:
  1 <= n <= 20
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 1 ms, faster than 73.91% of Rust online submissions for Spiral Matrix II.
  - Memory Usage: 2 MB, less than 100.00% of Rust online submissions for Spiral Matrix II.
   */
  pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
    let n = n as usize;
    let mut matrix = vec![vec![0;n];n];
    let mut v = 0;
    let mut write = |x: &mut i32| { v += 1; *x = v };
    let (mut u, mut d) = (0, n - 1);
    let (mut l, mut r) = (0, n - 1);
    loop {
      for x in matrix[u][l..=r].iter_mut() {
        write(x);
      }
      if u < d { u += 1 } else { break; };
      for row in matrix[u..=d].iter_mut() {
        write(&mut row[r]);
      }
      if l < r { r -= 1 } else { break; };
      for x in matrix[d][l..=r].iter_mut().rev() {
        write(x);
      }
      if u < d { d -= 1 } else { break; };
      for row in matrix[u..=d].iter_mut().rev() {
        write(&mut row[l]);
      }
      if l < r { l += 1 } else { break; };
    }
    matrix
  }
}
