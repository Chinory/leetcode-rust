/*
22. Generate Parentheses
Medium

Given n pairs of parentheses, write a function
 to generate all combinations of well-formed parentheses.

Example 1:
  Input: n = 3
  Output: ["((()))","(()())","(())()","()(())","()()()"]

Example 2:
  Input: n = 1
  Output: ["()"]

Constraints:
  1 <= n <= 8
*/
use crate::data::Solution;
impl Solution {
  pub fn generate_parenthesis(n: i32) -> Vec<String> {
    struct St { ans: Vec<String>, chs: Vec<u8> }
    impl St {
      fn dfs(&mut self, l:i32, r:i32) {
        if l==0 && r==0 {
          self.ans.push(unsafe{String::from_utf8_unchecked(self.chs.clone())});
        }
        if l != 0 {
          self.chs.push(b'(');
          self.dfs(l-1,r);
          self.chs.pop();
        }
        if l < r {
          self.chs.push(b')');
          self.dfs(l,r-1);
          self.chs.pop();
        }
      }
    }
    let mut st = St { ans: vec![], chs: Vec::with_capacity(n as usize * 2) };
    st.dfs(n,n);
    st.ans
  }
  pub fn generate_parenthesis_dp(n: i32) -> Vec<String> {
    if n == 0 { return vec![] }
    let n = n as usize;
    let mut dp = vec![
      vec!["".to_string()],
      vec!["()".to_string()]
    ];
    for i in 2..=n {
      let mut dpi = vec![];
      for j in 0..i {
        for p in dp[j].iter() {
          for q in dp[i-1-j].iter() {
            dpi.push("(".to_string() + p + ")" + q);
          }
        }
      }
      dp.push(dpi);
    }
    dp.pop().unwrap()
  }
}
