/*
51. N-Queens
Hard

The n-queens puzzle is the problem of placing n queens on an n x n chessboard such that no two queens attack each other.

Given an integer n, return all distinct solutions to the n-queens puzzle. You may return the answer in any order.

Each solution contains a distinct board configuration of the n-queens' placement, where 'Q' and '.' both indicate a queen and an empty space, respectively.

Example 1:
  Input: n = 4
  Output: [[".Q..","...Q","Q...","..Q."],["..Q.","Q...","...Q",".Q.."]]
  Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above

Example 2:
  Input: n = 1
  Output: [["Q"]]

Constraints:
  1 <= n <= 9
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 2 ms, faster than 85.09% of Rust online submissions for N-Queens.
  - Memory Usage: 2.3 MB, less than 94.41% of Rust online submissions for N-Queens.
   */
  pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
    struct State {
      queens: Vec<i32>,
      ans: Vec<Vec<String>>,
      n: i32, all: i32
    }
    fn make_board(queens: &Vec<i32>) -> Vec<String> {
      let mut board = Vec::with_capacity(queens.len());
      for &pos in queens {
        let mut row = vec![b'.'; queens.len()];
        row[pos as usize] = b'Q';
        board.push(unsafe{String::from_utf8_unchecked(row)});
      }
      board
    }
    fn backtrace(s: &mut State, row: i32, columns: i32, diagonals1: i32, diagonals2: i32) {
      if row == s.n {
        return s.ans.push(make_board(&s.queens));
      }
      let mut avail_columns = s.all & !(columns | diagonals1 | diagonals2);
      while avail_columns != 0 {
        let last_column = avail_columns & -avail_columns;
        avail_columns &= avail_columns - 1;
        s.queens[row as usize] = (last_column - 1).count_ones() as i32;
        backtrace(s, row + 1, columns | last_column,
                  (diagonals1 | last_column) << 1,
                  (diagonals2 | last_column) >> 1);
        s.queens[row as usize] = -1;
      }
    }
    let mut s = State {
      queens: vec![-1; n as usize] , ans: vec![], n, all: (1 << n) - 1
    };
    backtrace(&mut s, 0, 0, 0, 0);
    s.ans
  }
}
/*
遍历可以放置皇后的位置时，可以利用以下两个按位与运算的性质：
  x & (-x) 可以获得 x 的二进制表示中的最低位的 1 的位置；
  x & (x - 1) 可以将 x 的二进制表示中的最低位的 1 置成 0。

作者：LeetCode-Solution
链接：https://leetcode.cn/problems/n-queens/solution/nhuang-hou-by-leetcode-solution/
来源：力扣（LeetCode）
著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
*/
