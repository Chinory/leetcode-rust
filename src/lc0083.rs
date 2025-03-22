/*
83. Remove Duplicates from Sorted List
Easy

Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.

Example 1:
  Input: head = [1,1,2]
  Output: [1,2]

Example 2:
  Input: head = [1,1,2,3,3]
  Output: [1,2,3]

Constraints:
  The number of nodes in the list is in the range [0, 300].
  -100 <= Node.val <= 100
  The list is guaranteed to be sorted in ascending order.
*/
use crate::data::{ListNode, Solution};
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Remove Duplicates from Sorted List.
  - Memory Usage: 2 MB, less than 98.80% of Rust online submissions for Remove Duplicates from Sorted List.
   */
  pub fn delete_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut done = head?;
    let mut prev = done.val;
    let mut head = done.next.take();
    let mut tail = &mut done.next;
    while let Some(mut node) = head {
      head = node.next.take();
      if prev != node.val {
        prev = node.val;
        tail = &mut tail.insert(node).next;
      }
    }
    Some(done)
  }
}
