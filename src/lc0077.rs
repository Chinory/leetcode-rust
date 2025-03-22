/*
77. Combinations
Medium

Given two integers n and k, return all possible combinations of k numbers out of the range [1, n].

You may return the answer in any order.

Example 1:
  Input: n = 4, k = 2
  Output:
  [
    [2,4],
    [3,4],
    [2,3],
    [1,2],
    [1,3],
    [1,4],
  ]

Example 2:
  Input: n = 1, k = 1
  Output: [[1]]

Constraints:
  1 <= n <= 20
  1 <= k <= n
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 6 ms, faster than 91.18% of Rust online submissions for Combinations.
  - Memory Usage: 2.8 MB, less than 72.06% of Rust online submissions for Combinations.
   */
  pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
    struct State {
      i: i32, n: i32, k: i32,
      cur: Vec<i32>, ans: Vec<Vec<i32>>
    }
    fn dfs(s: &mut State) {
      if s.k - s.cur.len() as i32 > s.n - s.i {
        //
      } else if s.k - s.cur.len() as i32 > 0 {
        s.i += 1;
        s.cur.push(s.i);
        dfs(s);
        s.cur.pop();
        dfs(s);
        s.i -= 1;
      } else {
        s.ans.push(s.cur.clone());
      }
    }
    let mut s = State { i:0, n, k, cur: vec![], ans:vec![] };
    dfs(&mut s);
    s.ans
  }
}
