/*
6. Zigzag Conversion
Medium

The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)

P   A   H   N
A P L S I I G
Y   I   R
And then read line by line: "PAHNAPLSIIGYIR"
Write the code that will take a string and make this conversion given a number of rows:
string convert(string s, int numRows);

Example 1:
  Input: s = "PAYPALISHIRING", numRows = 3
  Output: "PAHNAPLSIIGYIR"

Example 2:
  Input: s = "PAYPALISHIRING", numRows = 4
  Output: "PINALSIGYAHRPI"
  Explanation:
  P     I    N
  A   L S  I G
  Y A   H R
  P     I

Example 3:
  Input: s = "A", numRows = 1
  Output: "A"

Constraints:
  1 <= s.length <= 1000
  s consists of English letters (lower-case and upper-case), ',' and '.'.
  1 <= numRows <= 1000
*/
use crate::data::Solution;
impl Solution {
  pub fn convert(s: String, num_rows: i32) -> String {
    let n = s.len();
    let r = num_rows as usize;
    if r == 1 || r >= n { return s }
    let mut ans = Vec::with_capacity(n);
    for _ in 0..n { ans.push(Vec::with_capacity(n>>1)) }
    let t = r * 2 - 2;
    let mut x = 0;
    for (i, &c) in s.as_bytes().into_iter().enumerate() {
      ans[x].push(c);
      if i % t < r - 1 { x += 1 } else { x -= 1 }
    }
    let ans = ans.into_iter().flatten().collect();
    unsafe { String::from_utf8_unchecked(ans) }
  }
}
