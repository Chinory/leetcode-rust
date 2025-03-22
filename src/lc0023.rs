/*
23. Merge k Sorted Lists
Hard

You are given an array of k linked-lists lists,
  each linked-list is sorted in ascending order.
Merge all the linked-lists into one sorted linked-list and return it.

Example 1:
  Input: lists = [[1,4,5],[1,3,4],[2,6]]
  Output: [1,1,2,3,4,4,5,6]
  Explanation: The linked-lists are:
  [
    1->4->5,
    1->3->4,
    2->6
  ]
  merging them into one sorted list:
  1->1->2->3->4->4->5->6

Example 2:
  Input: lists = []
  Output: []

Example 3:
  Input: lists = [[]]
  Output: []

Constraints:
  k == lists.length
  0 <= k <= 104
  0 <= lists[i].length <= 500
  -104 <= lists[i][j] <= 104
  lists[i] is sorted in ascending order.
  The sum of lists[i].length will not exceed 104.
*/
use crate::data::{ListNode, Solution};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

impl PartialOrd for ListNode {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for ListNode {
  fn cmp(&self, other: &Self) -> Ordering {
    other.val.cmp(&self.val)
  }
}

impl Solution {
  pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
    let mut ans = None;
    let mut cur = &mut ans;
    let mut heap = BinaryHeap::with_capacity(lists.len());
    for list in lists {
      if let Some(node) = list {
        heap.push(node);
      }
    }
    while let Some(mut node) = heap.pop() {
      if let Some(next) = node.next.take() {
        heap.push(next);
      }
      cur = &mut cur.insert(node).next;
    }
    ans
  }
}
