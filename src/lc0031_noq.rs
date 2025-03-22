use crate::data::Solution;
use core::mem;
impl Solution {
  pub fn next_permutation(nums: &mut Vec<i32>) {
    if Self::next_permutation_common(nums).is_none() {
      nums.reverse();
    }
  }
  pub fn next_permutation_common(a: &mut Vec<i32>) -> Option<()> {
    let (&mut mut hi, sl) = a.split_last_mut()?;
    let i = sl.into_iter().rposition(|&mut v| hi > v || { hi = v; false })?;
    let (lo, sl) = a[i..].split_first_mut()?;
    let hi = sl.iter_mut().rfind(|&&mut v| *lo < v)?;
    mem::swap(lo, hi);
    Some(sl.reverse())
  }
}
