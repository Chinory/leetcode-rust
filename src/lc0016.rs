/*
16. 3Sum Closest
Medium
Given an integer array nums of length n and an integer target,
 find three integers in nums such that the sum is closest to target.
Return the sum of the three integers.
You may assume that each input would have exactly one solution.
Example 1:
  Input: nums = [-1,2,1,-4], target = 1
  Output: 2
  Explanation: The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
Example 2:
  Input: nums = [0,0,0], target = 1
  Output: 0
Constraints:
  3 <= nums.length <= 1000
  -1000 <= nums[i] <= 1000
  -104 <= target <= 104
*/
use crate::Solution;
use std::slice::Iter;
impl Solution {
  pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut it = nums.iter().cloned();
    let mut ans = i32::MAX;
    let mut dif = i32::MAX;
    loop {
      let mut a =  match it.next() { Some(i) => i, _ => return ans };
      let mut bc = it.clone();
      let mut b = match bc.next() { Some(i) => i, _ => return ans };
      let mut c = match bc.next_back() { Some(i) => i, _ => return ans };
      'bc: loop {
        let sum = a + b + c;
        if sum > target {
          if dif > sum - target {
            dif = sum - target;
            ans = sum;
          }
          loop { match bc.next_back() {
            Some(i) => if c != i { break c = i },
            None => break 'bc
          } }
        } else if sum < target {
          if dif > target - sum {
            dif = target - sum;
            ans = sum;
          }
          loop { match bc.next() {
            Some(i) => if b != i { break b = i },
            None => break 'bc
          } }
        } else {
          return sum;
        }
      }
    }
  }
}
