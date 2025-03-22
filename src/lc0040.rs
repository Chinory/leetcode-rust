/*
40. Combination Sum II
Medium

Given a collection of candidate numbers (candidates) and a target number (target), find all unique combinations in candidates where the candidate numbers sum to target.

Each number in candidates may only be used once in the combination.

Note: The solution set must not contain duplicate combinations.

Example 1:
  Input: candidates = [10,1,2,7,6,1,5], target = 8
  Output:
  [
  [1,1,6],
  [1,2,5],
  [1,7],
  [2,6]
  ]

Example 2:
  Input: candidates = [2,5,2,1,2], target = 5
  Output:
  [
  [1,2,2],
  [5]
  ]

Constraints:
  1 <= candidates.length <= 100
  1 <= candidates[i] <= 50
  1 <= target <= 30
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Combination Sum II.
  - Memory Usage: 2.1 MB, less than 86.57% of Rust online submissions for Combination Sum II.
  */
  pub fn combination_sum2(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    struct State {
      nums: Vec<i32>, target: i32,
      cur: Vec<i32>, ans: Vec<Vec<i32>> }
    fn dfs(s: &mut State, mut i: usize) {
      if s.target == 0 { return s.ans.push(s.cur.clone()) }
      let mut prev = i32::MIN;
      while let Some(&num) = s.nums.get(i) {
        if num > s.target { break } else { i += 1 }
        if prev == num { continue } else { prev = num }
        s.target -= num;
        s.cur.push(num);
        dfs(s, i);
        s.cur.pop();
        s.target += num;
      }
    }
    let mut s = State { nums, target,
      cur: Vec::new(), ans: Vec::new() };
    s.nums.sort_unstable();
    dfs(&mut s, 0);
    s.ans
  }
}


#[test]
fn test() {
  let vec1 = Solution::combination_sum2(vec![10, 1, 2, 7, 6, 1, 5], 8);
  println!("{:?}",vec1);
}
