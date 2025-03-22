/*
206. Reverse Linked List
Easy

Given the head of a singly linked list, reverse the list, and return the reversed list.
*/
use crate::data::{ListNode, Solution};
use core::mem;
impl Solution {
  pub fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut answ = None;
    while let Some(mut node) = head {
      head = mem::replace(&mut node.next, answ);
      answ = Some(node);
    }
    answ
  }
}
