use crate::data::{ListNode, Solution};
use core::mem;
impl Solution {
  /**
    # Stack Version
  - disadvantage: space:O(k)
  - no unsafe
  - no unwrap
  - no repeat code
  - cleaner code
  - Rusty maybe
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Reverse Nodes in k-Group.
  - Memory Usage: 2.4 MB, less than 33.33% of Rust online submissions for Reverse Nodes in k-Group.
   */
  pub fn reverse_k_group(mut list: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k < 2 { return list; }
    let n = k as usize;
    let mut stack = Vec::with_capacity(n);
    let mut res_head = None;
    let mut res_last = &mut res_head;
    while let Some(mut node) = list {
      list = node.next.take();
      stack.push(node);
      if stack.len() < n { continue }
      while let Some(node) = stack.pop() {
        res_last = &mut res_last.insert(node).next;
      }
    }
    for node in stack {
      res_last = &mut res_last.insert(node).next;
    }
    res_head
  }
  /**
    # Classic Version
    - no unwrap
    - no repeat code
    - minial unsafe
   */
  pub fn reverse_k_group__classic(mut list: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if k < 2 { return list; }
    let mut prev = &mut list;
    while let Some(mut head) = prev.take() {
      let mut last = &mut head;
      for _ in 1..k {
        if let Some(next) = last.next.as_mut() {
          last = next;
        } else {
          *prev = Some(head);
          return list;
        }
      }
      let tail = last.next.take();
      let mut next = mem::replace(&mut head.next, tail);
      let new_prev: *mut _ = &mut head.next;
      while let Some(mut node) = next { // consume next->...
        next = mem::replace(&mut node.next, Some(head));
        head = node;
      }
      *prev = Some(head);
      // SAFETY: This is a pointer into a Box that never moves
      prev = unsafe { &mut *new_prev };
    }
    list
  }
}
