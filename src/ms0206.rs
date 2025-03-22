/*
面试题 02.06. 回文链表
编写一个函数，检查输入的链表是否是回文的。

示例 1：
  输入： 1->2
  输出： false

示例 2：
  输入： 1->2->2->1
  输出： true

进阶：
  你能否用 O(n) 时间复杂度和 O(1) 空间复杂度解决此题？
*/
use crate::data::{ListNode, Solution};
impl Solution {
  pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
    let mut vals = vec![];
    let mut cur = &head;
    while let Some(node) = cur {
      vals.push(node.val);
      cur = &node.next;
    }
    let mut it = vals.into_iter();
    while let (Some(l),Some(r)) = (it.next(), it.next_back()) {
      if l != r {
        return false;
      }
    }
    true
  }

  pub fn is_palindrome_jianghu(mut head: Option<Box<ListNode>>) -> bool {
    use core::mem;
    // 慢：反转链表
    // 快：碰头
    // 1 2 1
    //浆糊啊这也能写？？
    let mut rev0 = None;
    let mut slow = head;
    let mut fast: *mut _ = &mut slow;
    let mut mid;
    let mut rest: *mut _;
    loop {/////
      match unsafe { (*fast).as_mut() } {
        Some(r) => fast = &mut r.next,
        None => {
          mid = slow;
          fast = &mut mid;
          break; //第一次推进失败，这剩余的数量是偶数
        }
      }
      match unsafe { (*fast).as_mut() } {
        Some(r) => fast = &mut r.next,
        None => {
          let mut node = slow.unwrap();
          slow = node.next.take();
          mid = Some(node);
          break;  // 第二次推进失败，剩余是奇数个，而这slow指向的正是中间
        }
      }
      let mut node = slow.unwrap();
      slow = mem::replace(&mut node.next, rev0);
      rev0 = Some(node);
    };
    //我感觉这一通操作猛如虎，最终也还是必须要写入O(2n)个机器字节长，真不如申请一块线性内存好
    //反正是“简单”题嘛
    println!("here, {:?}", mid);
    let mut ans = true;

    ans
  }
}

#[test]
fn test() {
  Solution::is_palindrome(ListNode::from(vec![1, 2, 4, 5]));
}
