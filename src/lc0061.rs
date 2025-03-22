/*
61. Rotate List
Medium

Given the head of a linked list, rotate the list to the right by k places.

Example 1:
  Input: head = [1,2,3,4,5], k = 2
  Output: [4,5,1,2,3]

Example 2:
  Input: head = [0,1,2], k = 4
  Output: [2,0,1]

Constraints:
  The number of nodes in the list is in the range [0, 500].
  -100 <= Node.val <= 100
  0 <= k <= 2 * 109
*/
use crate::data::{ListNode, Solution};
impl Solution {
  /**
   # Best
   - minial unsafe
   - 执行用时：0 ms, 在所有 Rust 提交中击败了 100.00% 的用户
   - 内存消耗：1.9 MB, 在所有 Rust 提交中击败了 95.24% 的用户
   */
  pub fn rotate_right(mut head: Option<Box<ListNode>>, mut k: i32) -> Option<Box<ListNode>> {
    if k < 1 { return head; }
    let mut n = 0;
    let mut finder = &mut head;
    while let Some(node) = finder {
      finder = &mut node.next;
      n += 1;
    }
    if n < 2 { return head; }
    k %= n;
    if k < 1 { return head; }
    let mover: *mut _ = finder;
    let mut taker = &mut head;
    for _ in k..n {
      taker = &mut taker.as_mut().unwrap().next;
    }
    let new = taker.take();
    unsafe { *mover = head; }
    new
  }
}
