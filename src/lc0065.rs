/*
65. Valid Number
Hard

A valid number can be split up into these components (in order):

A decimal number or an integer.
(Optional) An 'e' or 'E', followed by an integer.
A decimal number can be split up into these components (in order):

(Optional) A sign character (either '+' or '-').
One of the following formats:
One or more digits, followed by a dot '.'.
One or more digits, followed by a dot '.', followed by one or more digits.
A dot '.', followed by one or more digits.
An integer can be split up into these components (in order):

(Optional) A sign character (either '+' or '-').
One or more digits.
For example, all the following are valid numbers: ["2", "0089", "-0.1", "+3.14", "4.", "-.9", "2e10", "-90E3", "3e+7", "+6e-1", "53.5e93", "-123.456e789"], while the following are not valid numbers: ["abc", "1a", "1e", "e3", "99e2.5", "--6", "-+3", "95a54e53"].

Given a string s, return true if s is a valid number.

Example 1:
  Input: s = "0"
  Output: true

Example 2:
  Input: s = "e"
  Output: false

Example 3:
  Input: s = "."
  Output: false

Constraints:
  1 <= s.length <= 20
  s consists of only English letters (both uppercase and lowercase), digits (0-9), plus '+', minus '-', or dot '.'.
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Valid Number.
  - Memory Usage: 2.1 MB, less than 77.27% of Rust online submissions for Valid Number.
   */
  pub fn is_number(s: String) -> bool {
    s.into_bytes().into_iter()
      .try_fold(State::Start, State::handle)
      .as_ref()
      .map_or(false, State::is_valid_end)
  }
}

enum State {
  Start,
  Sign,
  Integer,
  Dot,
  EmptyDot,
  Decimal,
  E,
  ExpSign,
  Exponent,
  End,
}

impl State {
  pub fn is_valid_end(&self) -> bool {
    use State::*;
    match self {
      Start | Sign | E | ExpSign | EmptyDot => false,
      _ => true,
    }
  }

  pub fn handle(self, c: u8) -> Option<State> {
    use State::*;
    match self {
      Start => match c {
        b' ' => Some(Start),
        b'+' | b'-' => Some(Sign),
        b'0'..=b'9' => Some(Integer),
        b'.' => Some(EmptyDot),
        _ => None,
      }
      Sign => match c {
        b'0'..=b'9' => Some(Integer),
        b'.' => Some(EmptyDot),
        _ => None,
      }
      Integer => match c {
        b'0'..=b'9' => Some(Integer),
        b'.' => Some(Dot),
        b'e' | b'E' => Some(E),
        b' ' => Some(End),
        _ => None,
      }
      EmptyDot => match c {
        b'0'..=b'9' => Some(Decimal), // "  .1" or "  +.1"
        _ => None,
      }
      Dot => match c {
        b'0'..=b'9' => Some(Decimal),
        b'e' | b'E' => Some(E),   // "46.e3"
        b' ' => Some(End),
        _ => None,
      }
      Decimal => match c {
        b'0'..=b'9' => Some(Decimal),
        b'e' | b'E' => Some(E),
        b' ' => Some(End),
        _ => None,
      }
      E => match c {
        b'+' | b'-' => Some(ExpSign),
        b'0'..=b'9' => Some(Exponent),
        _ => None,
      }
      ExpSign => match c {
        b'0'..=b'9' => Some(Exponent),
        _ => None,
      }
      Exponent => match c {
        b'0'..=b'9' => Some(Exponent),
        b' ' => Some(End),
        _ => None,
      }
      End => match c {
        b' ' => Some(End),
        _ => None,
      }
    }
  }
}
