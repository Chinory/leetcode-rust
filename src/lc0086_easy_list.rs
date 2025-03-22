/*
86. Partition List
Medium

Given the head of a linked list and a value x, partition it such that all nodes less than x come before nodes greater than or equal to x.

You should preserve the original relative order of the nodes in each of the two partitions.

Example 1:
  Input: head = [1,4,3,2,5,2], x = 3
  Output: [1,2,2,4,3,5]

Example 2:
  Input: head = [2,1], x = 2
  Output: [1,2]

Constraints:
  The number of nodes in the list is in the range [0, 200].
  -100 <= Node.val <= 100
  -200 <= x <= 200
*/
use crate::data::{ListNode, Solution};
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Partition List.
  - Memory Usage: 2 MB, less than 100.00% of Rust online submissions for Partition List.
   */
  pub fn partition(mut head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
    let mut less = None;
    let mut les = &mut less;
    let mut more = None;
    let mut mor = &mut more;
    while let Some(mut node) = head {
      head = node.next.take();
      if node.val < x {
        les = &mut les.insert(node).next;
      } else {
        mor = &mut mor.insert(node).next;
      }
    }
    *les = more;
    less
  }
}
