/*
55. Jump Game
Medium

You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position.

Return true if you can reach the last index, or false otherwise.

Example 1:
  Input: nums = [2,3,1,1,4]
  Output: true
  Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.

Example 2:
  Input: nums = [3,2,1,0,4]
  Output: false
  Explanation: You will always arrive at index 3 no matter what. Its maximum jump length is 0, which makes it impossible to reach the last index.

Constraints:
  1 <= nums.length <= 104
  0 <= nums[i] <= 105
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 2 ms, faster than 100.00% of Rust online submissions for Jump Game.
  - Memory Usage: 2.3 MB, less than 42.04% of Rust online submissions for Jump Game.
   */
  pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut j = 0;
    for (i, &n) in nums.iter().enumerate() {
      if i > j { return false }
      j = j.max(i + n as usize);
    }
    true
  }
}
