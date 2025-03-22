/*
24. Swap Nodes in Pairs
Medium

Given a linked list, swap every two adjacent nodes and return its head.
 You must solve the problem without modifying the values in the list's nodes
  (i.e., only nodes themselves may be changed.)

Example 1:
  Input: head = [1,2,3,4]
  Output: [2,1,4,3]

Example 2:
  Input: head = []
  Output: []

Example 3:
  Input: head = [1]
  Output: [1]

Constraints:
  The number of nodes in the list is in the range [0, 100].
  0 <= Node.val <= 100
*/
use crate::data::{ListNode, Solution};
impl Solution {
  /*
    Runtime: 0 ms, faster than 100.00% of Rust
     online submissions for Swap Nodes in Pairs.
    Memory Usage: 2.1 MB, less than 42.11% of Rust
     online submissions for Swap Nodes in Pairs.
  */
  pub fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev_next = &mut head;
    while let Some(mut curr) = prev_next.take() {
      if let Some(mut next) = curr.next.take() {
        let next = prev_next.insert(next);
        let next_next = next.next.take();
        let curr = next.next.insert(curr);
        curr.next = next_next;
        prev_next = &mut curr.next;
      } else {
        *prev_next = Some(curr);
        break;
      }
    }
    head
  }
}
