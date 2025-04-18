/*
面试题 01.08. 零矩阵
编写一种算法，若M × N矩阵中某个元素为0，则将其所在的行与列清零。

示例 1：
  输入：
  [
    [1,1,1],
    [1,0,1],
    [1,1,1]
  ]
  输出：
  [
    [1,0,1],
    [0,0,0],
    [1,0,1]
  ]

示例 2：
  输入：
  [
    [0,1,2,0],
    [3,4,5,2],
    [1,3,1,5]
  ]
  输出：
  [
    [0,0,0,0],
    [0,4,5,0],
    [0,3,1,0]
  ]
*/
use crate::data::Solution;
impl Solution {
  pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
    let mut first_col_zero = false; //matrix.iter().any(|r|r[0]==0);
    for i in 0..matrix.len() {
      if matrix[i][0] == 0 {
        first_col_zero = true;
      }
      for j in 1..matrix[i].len() {
        if matrix[i][j] == 0 {
          matrix[i][0] = 0;
          matrix[0][j] = 0;
        }
      }
    }
    for i in (0..matrix.len()).rev() {
      for j in 1..matrix[i].len() {
        if matrix[i][0] == 0 || matrix[0][j] == 0 {
          matrix[i][j] = 0;
        }
      }
      if first_col_zero {
        matrix[i][0] = 0;
      }
    }
  }
}
