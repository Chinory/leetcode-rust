/*
768. Max Chunks To Make Sorted II
Hard

You are given an integer array arr.
We split arr into some number of chunks (i.e., partitions), and individually sort each chunk.
After concatenating them, the result should equal the sorted array.
Return the largest number of chunks we can make to sort the array.

Example 1:
  Input: arr = [5,4,3,2,1]
  Output: 1
  Explanation:
  Splitting into two or more chunks will not return the required result.
  For example, splitting into [5, 4], [3, 2, 1] will result in [4, 5, 1, 2, 3], which isn't sorted.

Example 2:
  Input: arr = [2,1,3,4,4]
  Output: 4
  Explanation:
  We can split into two chunks, such as [2, 1], [3, 4, 4].
  However, splitting into [2, 1], [3], [4], [4] is the highest number of chunks possible.

Constraints:
  1 <= arr.length <= 2000
  0 <= arr[i] <= 108
*/
use crate::data::Solution;
impl Solution {
  pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
    let mut max = i32::MIN;
    let max: Vec<i32> = arr.iter().map(
      |&v| { if max < v { max = v; v } else { max } }).collect();
    let mut min = i32::MAX;
    let min: Vec<i32> = arr.iter().rev().map(
      |&v| { if min > v { min = v; v } else { min } }).collect();
    // if the max in prefix of current position
    // less than the min in suffix of next position,
    // so we can split at this position
    let mut it = min.into_iter().rev();
    it.next();
    it.zip(max.into_iter()).filter(|(min, max)| min >= max).count() as i32 + 1
  }
}
