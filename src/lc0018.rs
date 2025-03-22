/*
18. 4Sum
Medium

Given an array nums of n integers, return an array of all the unique quadruplets [nums[a], nums[b], nums[c], nums[d]] such that:

0 <= a, b, c, d < n
a, b, c, and d are distinct.
nums[a] + nums[b] + nums[c] + nums[d] == target
You may return the answer in any order.

Example 1:
  Input: nums = [1,0,-1,0,-2,2], target = 0
  Output: [[-2,-1,1,2],[-2,0,0,2],[-1,0,0,1]]

Example 2:
  Input: nums = [2,2,2,2,2], target = 8
  Output: [[2,2,2,2]]

Constraints:
  1 <= nums.length <= 200
  -109 <= nums[i] <= 109
  -109 <= target <= 109
*/
use crate::data::Solution;
impl Solution {
  pub fn four_sum(mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let n = nums.len();
    if n < 4 { return ans }
    nums.sort_unstable();
    let x = |i| nums[i] as i64;
    let T = target as i64;
    let xe2 = x(n-1) + x(n-2);
    let xe3 = xe2 + x(n-3);
    for a in 0..n-3 {
      if a>0 && x(a)==x(a-1) || x(a) + xe3 < T { continue }
      if x(a) + x(a+1) + x(a+2) + x(a+3) > T { break }
      let xe2a = xe2 + x(a);
      for b in a+1..n-2 {
        if b>a+1 && x(b)==x(b-1) || x(b) + xe2a < T { continue }
        if x(a) + x(b) + x(b+1) + x(b+2) > T { break }
        let xab = x(a) + x(b);
        let (mut c, mut d) = (b+1, n-1);
        while c < d {
          let x4 = xab + x(c) + x(d);
          if x4<T {
            c += 1;
          } else if x4>T {
            d -= 1;
          } else {
            ans.push(vec![nums[a],nums[b],nums[c],nums[d]]);
            while c<d && x(c)==x(c+1) { c += 1 } c += 1;
            while c<d && x(d)==x(d-1) { d -= 1 } d -= 1;
          }
        }
      }
    }
    ans
  }
}
