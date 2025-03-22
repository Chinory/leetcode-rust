/**
118. Pascal's Triangle
Easy

Given an integer numRows, return the first numRows of Pascal's triangle.

In Pascal's triangle, each number is the sum of the two numbers directly above it as shown:

Example 1:
    Input: numRows = 5
    Output: [[1],[1,1],[1,2,1],[1,3,3,1],[1,4,6,4,1]]

Example 2:
    Input: numRows = 1
    Output: [[1]]
Constraints:
    1 <= numRows <= 30
 */
use crate::data::Solution;
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        if num_rows < 1 { return vec![] }
        let n = num_rows as usize;
        let mut ans = Vec::with_capacity(n);
        ans.push(vec![1]);
        for i in 1..n {
            let mut pre = ans.last_mut().unwrap();
            let mut cur = vec![0; i + 1];
            cur[0] = 1; cur[i] = 1;
            for j in 1..i {
                cur[j] = pre[j] + pre[j - 1];
            }
            ans.push(cur);
        }
        ans
    }
}
