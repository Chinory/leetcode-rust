/*
15. 3Sum
Medium

Given an integer array nums, return all the triplets [nums[i], nums[j], nums[k]] such that i != j, i != k, and j != k, and nums[i] + nums[j] + nums[k] == 0.
Notice that the solution set must not contain duplicate triplets.

Example 1:
  Input: nums = [-1,0,1,2,-1,-4]
  Output: [[-1,-1,2],[-1,0,1]]

Example 2:
  Input: nums = []
  Output: []

Example 3:
  Input: nums = [0]
  Output: []

Constraints:
  0 <= nums.length <= 3000
  -105 <= nums[i] <= 105
*/
use crate::data::Solution;
impl Solution {
  pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort_unstable();
    let mut res = vec![];
    let mut A = i32::MIN; // last a
    let mut B = i32::MIN; // last b
    let mut i = nums.iter();
    while let Some(&a) = i.next() {
      if a > 0 { break }
      if a == A { continue } else { A = a }
      let mut j = i.clone();
      while let Some(&b) = j.next() {
        if b == B { continue } else { B = b }
        let c = -(a + b);
        if b > 0 && c < 0 { break }
        if j.as_slice().binary_search(&c).is_ok() {
          res.push(vec![a,b,c]);
        }
      }
    }
    res
  }
}
