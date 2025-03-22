/*
82. Remove Duplicates from Sorted List II
Medium

Given the head of a sorted linked list, delete all nodes that have duplicate numbers, leaving only distinct numbers from the original list. Return the linked list sorted as well.

Example 1:
  Input: head = [1,2,3,3,4,4,5]
  Output: [1,2,5]

Example 2:
  Input: head = [1,1,1,2,3]
  Output: [2,3]

Constraints:
  The number of nodes in the list is in the range [0, 300].
  -100 <= Node.val <= 100
  The list is guaranteed to be sorted in ascending order.
*/
use crate::data::{ListNode, Solution};
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Remove Duplicates from Sorted List II.
  - Memory Usage: 2.3 MB, less than 20.59% of Rust online submissions for Remove Duplicates from Sorted List II.
   */
  pub fn delete_duplicates2(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut tmp1 = head?;
    head = tmp1.next.take();
    let mut ans = None;
    let mut res = &mut ans;
    's: while let Some(mut tmp2) = head {
      head = tmp2.next.take();
      if tmp1.val != tmp2.val {
        res = &mut res.insert(tmp1).next;
        tmp1 = tmp2;
        continue 's;
      }
      while let Some(mut tmp3) = head {
        head = tmp3.next.take();
        if tmp2.val != tmp3.val {
          tmp1 = tmp3;
          continue 's;
        }
        tmp2 = tmp3;
      }
      return ans;
    }
    res = &mut res.insert(tmp1).next;
    ans
  }
}

#[test]
fn test() {
  let mut list = ListNode::from(vec![1, 1]);
  Solution::delete_duplicates2(list);
}
