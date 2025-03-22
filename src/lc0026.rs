/*
26. Remove Duplicates from Sorted Array
Easy

Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same.
Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
Return k after placing the final result in the first k slots of nums.
Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
*/
use crate::data::Solution;
impl Solution {
  /** Rusty */
  pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut last = i32::MIN;
    nums.retain(|&num| { if last == num { false } else { last = num; true } });
    nums.len() as i32
  }
  /** Classic */
  pub fn remove_duplicates_classic(nums: &mut Vec<i32>) -> i32 {
    if nums.is_empty() { return 0 }
    let mut i = 0;
    let mut j = 1;
    while let Some(&b) = nums.get(j) {
      if nums[i] != nums[j] {
        i += 1;
        nums[i] = nums[j];
      }
      j += 1;
    }
    i as i32 + 1
  }
