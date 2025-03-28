/*
27. Remove Element
Easy

Given an integer array nums and an integer val, remove all occurrences of val in nums in-place. The relative order of the elements may be changed.
Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the first part of the array nums. More formally, if there are k elements after removing the duplicates, then the first k elements of nums should hold the final result. It does not matter what you leave beyond the first k elements.
Return k after placing the final result in the first k slots of nums.
Do not allocate extra space for another array. You must do this by modifying the input array in-place with O(1) extra memory.
*/
use crate::data::Solution;
impl Solution {
  /** Rusty */
  pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|&n|n!=val);
    nums.len() as i32
  }
  /** Classic */
  pub fn remove_element_classic(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut index = 0;
    for i in 0..nums.len(){
      if nums[i] != val {
        nums[index] = nums[i];
        index += 1;
      }
    }
    return index as i32;
  }
}
