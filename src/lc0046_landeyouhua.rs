/*
46. Permutations
Medium

Given an array nums of distinct integers, return all the possible permutations. You can return the answer in any order.

Example 1:
  Input: nums = [1,2,3]
  Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

Example 2:
  Input: nums = [0,1]
  Output: [[0,1],[1,0]]

Example 3:
  Input: nums = [1]
  Output: [[1]]

Constraints:
  1 <= nums.length <= 6
  -10 <= nums[i] <= 10
  All the integers of nums are unique.
*/
use crate::data::Solution;
impl Solution {
  pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
    struct State {
      cur: Vec<i32>,
      ans: Vec<Vec<i32>> }
    fn dfs(s: &mut State, first: usize) {
      if first == s.cur.len() {
        return s.ans.push(s.cur.clone());
      }
      for i in first..s.cur.len() {
        s.cur.swap(first, i);
        dfs(s, first + 1);
        s.cur.swap(first, i);
      }
    }
    let mut s = State {
      cur: nums,
      ans: Vec::new() };
    dfs(&mut s, 0);
    s.ans
  }
}

