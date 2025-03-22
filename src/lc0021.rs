/*
21. Merge Two Sorted Lists
Easy

You are given the heads of two sorted linked lists list1 and list2.
Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
Return the head of the merged linked list.

Example 1:
  Input: list1 = [1,2,4], list2 = [1,3,4]
  Output: [1,1,2,3,4,4]

Example 2:
  Input: list1 = [], list2 = []
  Output: []

Example 3:
  Input: list1 = [], list2 = [0]
  Output: [0]

Constraints:
  The number of nodes in both lists is in the range [0, 50].
  -100 <= Node.val <= 100
  Both list1 and list2 are sorted in non-decreasing order.
*/
use crate::data::ListNode;
use crate::Solution;
impl Solution {
  pub fn merge_two_lists(mut list1: Option<Box<ListNode>>, mut list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = ListNode::new(-1);
    let mut dst = &mut dummy;
    while let (Some(l1), Some(l2)) = (list1.as_ref(), list2.as_ref()) {
      let src = if l1.val <= l2.val { &mut list1 } else { &mut list2 };
      dst.next = src.take();
      dst = dst.next.as_mut().unwrap();
      *src = dst.next.take();
    }
    dst.next = list1.or(list2);
    dummy.next
  }
}
