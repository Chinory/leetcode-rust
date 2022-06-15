use std::cell::RefCell;
use std::rc::Rc;

#[derive(PartialEq, Eq, Clone, Debug)]
pub(crate) struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

#[allow(dead_code)]
impl ListNode {
  #[inline]
  #[allow(dead_code)]
  pub(crate) fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
  pub(crate) fn from(v: Vec<i32>) -> Option<Box<Self>> {
    let mut dummy = None;
    let mut cur = &mut dummy;
    for i in v {
      *cur = Some(Box::new(ListNode::new(i)));
      cur = &mut cur.as_mut().unwrap().next;
    }
    dummy
  }
  pub(crate) fn print(mut r: Option<Box<ListNode>>) {
    while let Some(b) = r {
      print!("{},",b.val);
      r = b.next;
    }
  }
}

// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  #[allow(dead_code)]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}
#[allow(dead_code)]
pub(crate) struct Solution{}
