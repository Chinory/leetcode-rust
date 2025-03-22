/*
1. Two Sum
Easy

Given an array of integers nums and an integer target, 
return indices of the two numbers such that they add up to target.
You may assume that each input would have exactly one solution, 
and you may not use the same element twice.
You can return the answer in any order.
*/
use crate::data::Solution;
use std::collections::HashMap;
impl Solution {
  pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    let mut j = 0;
    for curr in nums {
      if let Some(&i) = map.get(&(target - curr)) {
        return vec![i, j];
      }
      map.insert(curr, j);
      j += 1;
    }
    vec![-1,-1]
  }
}