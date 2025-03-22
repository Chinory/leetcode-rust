/*
75. Sort Colors
Medium

Given an array nums with n objects colored red, white, or blue, sort them in-place so that objects of the same color are adjacent, with the colors in the order red, white, and blue.

We will use the integers 0, 1, and 2 to represent the color red, white, and blue, respectively.

You must solve this problem without using the library's sort function.

Example 1:
  Input: nums = [2,0,2,1,1,0]
  Output: [0,0,1,1,2,2]

Example 2:
  Input: nums = [2,0,1]
  Output: [0,1,2]

Constraints:
  n == nums.length
  1 <= n <= 300
  nums[i] is either 0, 1, or 2.
*/
use crate::data::Solution;
impl Solution {
  pub fn sort_colors(nums: &mut Vec<i32>) {
    let mut cnt = [0u32; 3];
    for x in nums.iter() {
      cnt[*x as usize] += 1;
    }
    let mut it = nums.iter_mut();
    for _ in 0..cnt[0] { *it.next().unwrap() = 0; }
    for _ in 0..cnt[1] { *it.next().unwrap() = 1; }
    for _ in 0..cnt[2] { *it.next().unwrap() = 2; }
  }
}
