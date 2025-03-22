/*
19. Remove Nth Node From End of List
Medium

Given the head of a linked list, remove the nth node from the end of the list and return its head.

Example 1:
  Input: head = [1,2,3,4,5], n = 2
  Output: [1,2,3,5]

Example 2:
  Input: head = [1], n = 1
  Output: []

Example 3:
  Input: head = [1,2], n = 1
  Output: [1]

Constraints:
  The number of nodes in the list is sz.
  1 <= sz <= 30
  0 <= Node.val <= 100
  1 <= n <= sz
*/
use crate::data::ListNode;
use crate::Solution;
impl Solution {
  pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    unsafe {
      let mut slow = &mut dummy as *mut Box<ListNode>;
      let mut fast = &mut dummy as *mut Box<ListNode>;
      // move fast n forward
      for _ in 0..n {
        fast = (*fast).next.as_mut().unwrap();
      }
      while (*fast).next.is_some() {
        fast = (*fast).next.as_mut().unwrap();
        slow = (*slow).next.as_mut().unwrap();
      }
      (*slow).next = (*slow).next.take().unwrap().next;
    }
    dummy.next
  }
}
