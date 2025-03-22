/*
47. Permutations II
Medium

Given a collection of numbers, nums, that might contain duplicates, return all possible unique permutations in any order.

Example 1:
  Input: nums = [1,1,2]
  Output:
  [[1,1,2],
   [1,2,1],
   [2,1,1]]

Example 2:
  Input: nums = [1,2,3]
  Output: [[1,2,3],[1,3,2],[2,1,3],[2,3,1],[3,1,2],[3,2,1]]

Constraints:
  1 <= nums.length <= 8
  -10 <= nums[i] <= 10
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 4 ms, faster than 68.16% of Rust online submissions for Permutations II.
  - Memory Usage: 2.1 MB, less than 99.44% of Rust online submissions for Permutations II.
   */
  pub fn permute_unique(nums: Vec<i32>) -> Vec<Vec<i32>> {
    struct State {
      nums: Vec<i32>, used: Vec<bool>,
      cur: Vec<i32>, ans: Vec<Vec<i32>>,
    }
    fn dfs(s: &mut State) {
      if s.cur.len() == s.nums.len() {
        return s.ans.push(s.cur.clone());
      }
      let mut prev = i32::MIN;
      for i in 0..s.nums.len() {
        if s.used[i] || prev == s.nums[i] { continue }
        prev = s.nums[i];
        s.used[i] = true;
        s.cur.push(s.nums[i]);
        dfs(s);
        s.cur.pop();
        s.used[i] = false;
      }
    }
    let mut s = State {
      used: vec![false; nums.len()], nums,
      cur: Vec::new(), ans: Vec::new(),
    };
    s.nums.sort_unstable();
    dfs(&mut s);
    s.ans
  }
}
