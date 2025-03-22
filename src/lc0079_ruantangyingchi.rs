/*
79. Word Search
Medium

Given an m x n grid of characters board and a string word, return true if word exists in the grid.

The word can be constructed from letters of sequentially adjacent cells, where adjacent cells are horizontally or vertically neighboring. The same letter cell may not be used more than once.

Example 1:
  Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCCED"
  Output: true

Example 2:
  Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "SEE"
  Output: true

Example 3:
  Input: board = [["A","B","C","E"],["S","F","C","S"],["A","D","E","E"]], word = "ABCB"
  Output: false

Constraints:
  m == board.length
  n = board[i].length
  1 <= m, n <= 6
  1 <= word.length <= 15
  board and word consists of only lowercase and uppercase English letters.

Follow up: Could you use search pruning to make your solution faster with a larger board?
*/
use crate::data::Solution;
impl Solution {
  /**
  # 软糖硬吃版
  - Runtime: 125 ms, faster than 78.87% of Rust online submissions for Word Search.
  - Memory Usage: 2.1 MB, less than 78.87% of Rust online submissions for Word Search.
  - 执行用时：48 ms, 在所有 Rust 提交中击败了93.94%的用户
  - 内存消耗：1.9 MB, 在所有 Rust 提交中击败了95.45%的用户
   */
  pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
    #![allow(arithmetic_overflow)]
    fn dfs(board: &mut Vec<Vec<char>>, i: usize, j: usize, w: &[u8]) -> Option<()> {
      let (&c, w) = w.split_first()?;
      if visit(board, i, j, c).is_some() {
        dfs(board, i - 1, j, w)?;
        dfs(board, i, j - 1, w)?;
        dfs(board, i + 1, j, w)?;
        dfs(board, i, j + 1, w)?;
        board[i][j] = c as char;
      }
      Some(())
    }
    fn visit(board: &mut Vec<Vec<char>>, i: usize, j: usize, required: u8) -> Option<()> {
      let r = board.get_mut(i)?.get_mut(j)?;
      if *r as u8 == required { *r = '\0'; Some(()) } else { None }
    }
    let word = word.into_bytes();
    for i in 0..board.len() {
      for j in 0..board[i].len() {
        if dfs(&mut board, i, j, &word).is_none() {
          return true;
        }
      }
    }
    false
  }
}
