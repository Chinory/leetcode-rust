
use crate::data::{ListNode, Solution};
use core::mem;
impl Solution {
  pub fn reverse_between(mut head: Option<Box<ListNode>>, left: i32, right: i32) -> Option<Box<ListNode>> {
    let mut prev = &mut head;
    for _ in 1..left {
      prev = &mut prev.as_mut().unwrap().next;
    }
    let mut first = prev.take().unwrap();
    let mut rest = first.next.take();
    let mut last: *mut _ = &mut first.next;
    for _ in left..right {
      let mut node = rest.unwrap();
      rest = mem::replace(&mut node.next, Some(first));
      first = node;
    }

    head
  }
  /**
  写成了按值反转，寄
   */
  pub fn reverse_between_by_value(mut head: Option<Box<ListNode>>, left_val: i32, right_val: i32) -> Option<Box<ListNode>> {
    let mut left: *mut _ = &mut head;
    unsafe {
      loop {
        if let Some(node) = (*left).as_mut() { //这居然consume这个引用？？
          if node.val == left_val { break; }
          left = &mut node.next;
        } else {
          return head;
        }
      }
    }
    let mut rev_head = unsafe { (*left).take().unwrap() };
    let mut right = rev_head.next.take();
    let mut rev_tail: *mut _ = &mut rev_head.next; // this never move
    while let Some(mut node) = right {
      let is_right = node.val == right_val;
      right = mem::replace(&mut node.next, Some(rev_head));
      rev_head = node;
      if is_right {
        unsafe { *rev_tail = right };
        break;
      }
    }
    unsafe { *left = Some(rev_head) };
    head
  }
}
