/**
102. Binary Tree Level Order Traversal
Medium

Given the root of a binary tree, return the level order traversal of its nodes' values. (i.e., from left to right, level by level).

Example 1:
    Input: root = [3,9,20,null,null,15,7]
    Output: [[3],[9,20],[15,7]]

Example 2:
    Input: root = [1]
    Output: [[1]]

Example 3:
    Input: root = []
    Output: []

Constraints:
    The number of nodes in the tree is in the range [0, 2000].
    -1000 <= Node.val <= 1000
 */

use std::rc::Rc;
use std::cell::RefCell;
use crate::data::{Solution, TreeNode};

impl Solution {
    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut cur = match root {
            Some(n) => vec![n],
            _ => return vec![]
        };
        let mut ans = vec![];
        while !cur.is_empty() {
            let mut res = vec![];
            let mut nex = vec![];
            for x in cur {
                let mut r = x.borrow_mut();
                res.push(r.val);
                if let Some(n) = r.left.take() { nex.push(n); }
                if let Some(n) = r.right.take() { nex.push(n); }
            }
            cur = nex;
            ans.push(res);
        }
        ans
    }
}
