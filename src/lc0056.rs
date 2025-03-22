/*
56. Merge Intervals
Medium

Given an array of intervals where intervals[i] = [starti, endi], merge all overlapping intervals, and return an array of the non-overlapping intervals that cover all the intervals in the input.

Example 1:
  Input: intervals = [[1,3],[2,6],[8,10],[15,18]]
  Output: [[1,6],[8,10],[15,18]]
  Explanation: Since intervals [1,3] and [2,6] overlap, merge them into [1,6].

Example 2:
  Input: intervals = [[1,4],[4,5]]
  Output: [[1,5]]
  Explanation: Intervals [1,4] and [4,5] are considered overlapping.

Constraints:
  1 <= intervals.length <= 104
  intervals[i].length == 2
  0 <= starti <= endi <= 104
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 6 ms, faster than 82.86% of Rust online submissions for Merge Intervals.
  - Memory Usage: 2.9 MB, less than 98.57% of Rust online submissions for Merge Intervals.
   */
  pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut ans = vec![];
    let mut ranges: Vec<_> = intervals.into_iter()
      .map(|t| (t[0], t[1])).collect();
    ranges.sort_unstable();
    let mut it = ranges.into_iter();
    let (mut cl, mut cr) = it.next().unwrap(); //len>0
    for (l, r) in it {
      if cr < l {
        ans.push(vec![cl, cr]);
        cl = l;
      }
      if cr < r {
        cr = r;
      }
    }
    ans.push(vec![cl, cr]);
    ans
  }
}
