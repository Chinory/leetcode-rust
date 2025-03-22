/*
68. Text Justification
Hard

Given an array of strings words and a width maxWidth, format the text such that each line has exactly maxWidth characters and is fully (left and right) justified.

You should pack your words in a greedy approach; that is, pack as many words as you can in each line. Pad extra spaces ' ' when necessary so that each line has exactly maxWidth characters.

Extra spaces between words should be distributed as evenly as possible. If the number of spaces on a line does not divide evenly between words, the empty slots on the left will be assigned more spaces than the slots on the right.

For the last line of text, it should be left-justified, and no extra space is inserted between words.

Note:

A word is defined as a character sequence consisting of non-space characters only.
Each word's length is guaranteed to be greater than 0 and not exceed maxWidth.
The input array words contains at least one word.

Example 1:
  Input: words = ["This", "is", "an", "example", "of", "text", "justification."], maxWidth = 16
  Output:
  [
     "This    is    an",
     "example  of text",
     "justification.  "
  ]

Example 2:
  Input: words = ["What","must","be","acknowledgment","shall","be"], maxWidth = 16
  Output:
  [
    "What   must   be",
    "acknowledgment  ",
    "shall be        "
  ]
  Explanation: Note that the last line is "shall be    " instead of "shall     be", because the last line must be left-justified instead of fully-justified.
  Note that the second line is also left-justified because it contains only one word.

Example 3:
  Input: words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"], maxWidth = 20
  Output:
  [
    "Science  is  what we",
    "understand      well",
    "enough to explain to",
    "a  computer.  Art is",
    "everything  else  we",
    "do                  "
  ]

Constraints:
  1 <= words.length <= 300
  1 <= words[i].length <= 20
  words[i] consists of only English letters and symbols.
  1 <= maxWidth <= 100
  words[i].length <= maxWidth
*/
use crate::data::Solution;
impl Solution {
  /**
  - Runtime: 0 ms, faster than 100.00% of Rust online submissions for Text Justification.
  - Memory Usage: 2.1 MB, less than 52.94% of Rust online submissions for Text Justification.
   */
  pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
    let max = max_width as usize + 1;
    let mut lines = Vec::new();
    let mut line_words: Vec<String> = Vec::new();
    let mut line_len = 0;
    for word in words {
      if line_len + word.len() + 1 > max {
        let n = line_words.len();
        assert!(n > 0);
        let mut it = line_words.iter();
        let mut line = String::with_capacity(max-1);
        line.push_str(it.next().unwrap());
        let spaces = max - line_len;
        if n > 1 {
          let avg = spaces / (n - 1) + 2;
          for _ in 0..spaces % (n - 1) {
            for _ in 0..avg { line.push(' '); }
            line.push_str(it.next().unwrap());
          }
          for rest_word in it {
            for _ in 1..avg { line.push(' '); }
            line.push_str(rest_word);
          }
        } else {
          for _ in 0..spaces { line.push(' '); }
        }
        line_len = 0;
        line_words.clear();
        lines.push(line);
      }
      line_len += word.len() + 1;
      line_words.push(word);
    }
    let mut it = line_words.into_iter();
    if let Some(first_word) = it.next() {
      let mut last_line = String::with_capacity(max-1);
      last_line.push_str(&first_word);
      for word in it {
        last_line.push(' ');
        last_line.push_str(&word);
      }
      for _ in last_line.len()..max-1 {
        last_line.push(' ');
      }
      lines.push(last_line);
    }
    lines
  }
}

#[test]
fn test() {
  let words = ["Science","is","what","we","understand","well","enough","to","explain","to","a","computer.","Art","is","everything","else","we","do"].map(|s|s.to_owned()).to_vec();
  let maxWidth = 20;
  let vec = Solution::full_justify(words, maxWidth);
  for x in vec {
    println!("{:?} {:?}", x.capacity(), x.len());
  }

}
