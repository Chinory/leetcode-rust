/*
11. Container With Most Water
Medium

You are given an integer array height of length n. 
There are n vertical lines drawn such that 
the two endpoints of the ith line are (i, 0) and (i, height[i]).
Find two lines that together with the x-axis form a container, 
such that the container contains the most water.
Return the maximum amount of water a container can store.
Notice that you may not slant the container.
...
*/
use crate::data::Solution;
impl Solution {
  pub fn max_area(height: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut it = height.iter();
    let mut l = match it.next() { Some(&i)=>i, _=>return ans };
    let mut r = match it.next_back() { Some(&i)=>i, _=>return ans };
    loop {
      ans = ans.max(l.min(r) * (it.len()+1) as i32);
      if l < r { l = match it.next() { Some(&i)=>i, _=>return ans } }
      else { r = match it.next_back() { Some(&i)=>i, _=>return ans } }
    }
  }
}
