/*
33. Search in Rotated Sorted Array
Medium

There is an integer array nums sorted in ascending order (with distinct values).

Prior to being passed to your function, nums is possibly rotated at an unknown pivot index k (1 <= k < nums.length) such that the resulting array is [nums[k], nums[k+1], ..., nums[n-1], nums[0], nums[1], ..., nums[k-1]] (0-indexed). For example, [0,1,2,4,5,6,7] might be rotated at pivot index 3 and become [4,5,6,7,0,1,2].

Given the array nums after the possible rotation and an integer target, return the index of target if it is in nums, or -1 if it is not in nums.

You must write an algorithm with O(log n) runtime complexity.

Example 1:
  Input: nums = [4,5,6,7,0,1,2], target = 0
  Output: 4

Example 2:
  Input: nums = [4,5,6,7,0,1,2], target = 3
  Output: -1

Example 3:
  Input: nums = [1], target = 0
  Output: -1

Constraints:
  1 <= nums.length <= 5000
  -104 <= nums[i] <= 104
  All values of nums are unique.
  nums is an ascending array that is possibly rotated.
  -104 <= target <= 104
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Search in Rotated Sorted Array.
  - Memory Usage: 2.1 MB, less than 83.64% of Rust online submissions for Search in Rotated Sorted Array.
   */
  pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut prev = nums[0];
    let mut i = 1;
    while i < nums.len() {
      if prev > nums[i] {
        match nums[i..].binary_search(&target) {
          Ok(p) => return (i+p) as i32, _ => break
        }
      }
      prev = nums[i];
      i += 1;
    }
    nums[..i].binary_search(&target).map_or(-1,|p|p as i32)
  }
}

#[test]
fn test() {
  let i = Solution::search(vec![4, 5, 6, 7, 0, 1, 2], 0);
  println!("{:?}",i);
}
