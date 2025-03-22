/*
2. Add Two Numbers
Medium

You are given two non-empty linked lists representing two non-negative integers. 
The digits are stored in reverse order, and each of their nodes contains a single digit. 
Add the two numbers and return the sum as a linked list.
You may assume the two numbers do not contain any leading zero,
 except the number 0 itself.
*/
use crate::data::{ListNode, Solution};
impl Solution {
  pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>
  ) -> Option<Box<ListNode>> {
    let mut p1 = &mut l1;
    let mut car = false;
    while let Some(n2) = l2 {
      if let Some(r1) = p1 {
        let val = r1.val + n2.val + car as i32;
        car = val > 9;
        r1.val = if car { val - 10 } else { val };
        p1 = &mut r1.next;
        l2 = n2.next;
      } else {
        *p1 = Some(n2);
        break;
      }
    }
    if car {
      while let Some(r1) = p1 {
        if r1.val < 9 {
          r1.val += 1;
          return l1;
        } else {
          r1.val = 0;
          p1 = &mut r1.next;
        }
      }
      *p1 = Some(Box::new(ListNode::new(1)));
    }
    l1
  }
}
