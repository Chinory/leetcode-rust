/*
81. Search in Rotated Sorted Array II
Medium

There is an integer array nums sorted in non-decreasing order (not necessarily with distinct values).

Before being passed to your function, nums is rotated at an unknown pivot index k (0 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,4,4,5,6,6,7] might be rotated at pivot index 5 and become [4,5,6,6,7,0,1,2,4,4].

Given the array nums after the rotation and an integer target, return true if target is in nums, or false if it is not in nums.

You must decrease the overall operation steps as much as possible.

Example 1:
  Input: nums = [2,5,6,0,0,1,2], target = 0
  Output: true

Example 2:
  Input: nums = [2,5,6,0,0,1,2], target = 3
  Output: false

Constraints:
  1 <= nums.length <= 5000
  -104 <= nums[i] <= 104
  nums is guaranteed to be rotated at some pivot.
  -104 <= target <= 104

Follow up: This problem is similar to Search in Rotated Sorted Array, but nums may contain duplicates. Would this affect the runtime complexity? How and why?
*/

use crate::data::Solution;

impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Search in Rotated Sorted Array II.
  - Memory Usage: 2 MB, less than 80.00% of Rust online submissions for Search in Rotated Sorted Array II.
   */
  pub fn search(nums: Vec<i32>, target: i32) -> bool {
    let mut prev = i32::MIN;
    for (i, &v) in nums.iter().enumerate() {
      if prev > v {
        return match nums[..i].binary_search(&target) {
          Ok(_) => true,
          Err(_) => match nums[i..].binary_search(&target) {
            Ok(_) => true,
            Err(_) => false
          }
        };
      } else {
        prev = v;
      }
    }
    match nums.binary_search(&target) {
      Ok(_) => true,
      Err(_) => false
    }
  }
}

#[test]
fn test() {
  Solution::search(vec![2,5,6,0,0,1,2], 0);
}
