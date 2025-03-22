/*
面试题 02.05. 链表求和
给定两个用链表表示的整数，每个节点包含一个数位。

这些数位是反向存放的，也就是个位排在链表首部。

编写函数对这两个整数求和，并用链表形式返回结果。

示例：
  输入：(7 -> 1 -> 6) + (5 -> 9 -> 2)，即617 + 295
  输出：2 -> 1 -> 9，即912
  进阶：思考一下，假设这些数位是正向存放的，又该如何解决呢?

示例：
  输入：(6 -> 1 -> 7) + (2 -> 9 -> 5)，即617 + 295
  输出：9 -> 1 -> 2，即912
*/
use crate::data::{ListNode, Solution};
impl Solution {
  pub fn add_two_numbers(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
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
