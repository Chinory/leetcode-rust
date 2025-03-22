/*
57. Insert Interval
Medium

You are given an array of non-overlapping intervals intervals where intervals[i] = [starti, endi] represent the start and the end of the ith interval and intervals is sorted in ascending order by starti. You are also given an interval newInterval = [start, end] that represents the start and end of another interval.

Insert newInterval into intervals such that intervals is still sorted in ascending order by starti and intervals still does not have any overlapping intervals (merge overlapping intervals if necessary).

Return intervals after the insertion.

Example 1:
  Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
  Output: [[1,5],[6,9]]

Example 2:
  Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
  Output: [[1,2],[3,10],[12,16]]
  Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].

Constraints:
  0 <= intervals.length <= 104
  intervals[i].length == 2
  0 <= starti <= endi <= 105
  intervals is sorted by starti in ascending order.
  newInterval.length == 2
  0 <= start <= end <= 105
*/
use crate::data::Solution;
use std::cmp::Ordering;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Insert Interval.
  - Memory Usage: 2.7 MB, less than 82.61% of Rust online submissions for Insert Interval.
   */
  pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
    let left = new_interval[0];
    let right = new_interval[1];
    let i = match intervals.binary_search_by_key(&left, |t| t[1]) {
      Ok(i) => {
        intervals[i][1] = right;
        i
      }
      Err(i) => {
        if let Some(v) = intervals.get_mut(i) {
          if let [l, r] = v.as_mut_slice() {
            if *l > right {
              intervals.insert(i, new_interval);
              return intervals;
            }
            if *l > left { *l = left; }
            if *r >= right { return intervals; }
            *r = right;
            i
          } else { unreachable!(); }
        } else {
          intervals.push(new_interval);
          return intervals;
        }
      }
    };
    let mut j = i + 1;
    while let Some(v) = intervals.get_mut(j) {
      match v[1].cmp(&right) {
        Ordering::Less => j += 1,
        Ordering::Equal => {
          j += 1;
          break;
        }
        Ordering::Greater => {
          if v[0] <= right {
            intervals[i][1] = v[1];
            j += 1;
          }
          break;
        }
      }
    }
    intervals.drain(i+1..j);
    intervals
  }
}
