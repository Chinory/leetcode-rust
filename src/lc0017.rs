/*
17. Letter Combinations of a Phone Number
Medium

Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
A mapping of digit to letters (just like on the telephone buttons) is given below. Note that 1 does not map to any letters.
*/
use crate::data::Solution;
impl Solution {
  pub fn letter_combinations(digits: String) -> Vec<String> {
    fn dfs(ans: &mut Vec<String>, cur: &mut String, ss: &[&str]) {
      let (&s, ss) = match ss.split_first() {
        Some(s_ss) => s_ss,
        None => return ans.push(cur.clone())
      };
      for c in s.chars() {
        cur.push(c);
        dfs(ans, cur, ss);
        cur.pop();
      }
    }
    if digits.is_empty() { return vec![]; }
    let kbd = &["abc", "def", "ghi", "jkl", "mno", "pqrs", "tuv", "wxyz"];
    let vs: Vec<&str> = digits.chars().map(|c| kbd[c as usize - '2' as usize]).collect();
    let mut ans = Vec::new();
    let mut cur = String::new();
    dfs(&mut ans, &mut cur, vs.as_slice());
    ans
  }
}
