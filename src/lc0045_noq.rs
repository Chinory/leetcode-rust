use crate::data::Solution;
impl Solution {
  pub fn jump(mut nums: Vec<i32>) -> i32 {
    let mut min_step = 0;
    let mut reachable = 0;
    let mut there = 0;
    let mut decided = 0;
    nums.pop();
    for jump in nums {
      reachable = reachable.max(there + jump);
      if there == decided {
        decided = reachable;
        min_step += 1;
      }
      there += 1;
    }
    min_step
  }
}
