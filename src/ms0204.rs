/*
面试题 02.04. 分割链表
给你一个链表的头节点 head 和一个特定值 x ，请你对链表进行分隔，使得所有 小于 x 的节点都出现在 大于或等于 x 的节点之前。

你不需要 保留 每个分区中各节点的初始相对位置。

示例 1：
  输入：head = [1,4,3,2,5,2], x = 3
  输出：[1,2,2,4,3,5]

示例 2：
  输入：head = [2,1], x = 2
  输出：[1,2]

提示：
  链表中节点的数目在范围 [0, 200] 内
  -100 <= Node.val <= 100
  -200 <= x <= 200
*/
use crate::data::{ListNode, Solution};
impl Solution {
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
