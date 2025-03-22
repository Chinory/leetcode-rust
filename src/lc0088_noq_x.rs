impl Solution {
  pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m - 1;
    let mut j = n - 1;
    let mut k = m + n - 1;
    loop {
      let cur;
      if i < 0 {
        if j < 0 { break }
        cur = nums2[j as usize]; j -= 1;
      } else if j < 0 {
        cur = nums1[i as usize]; i -= 1;
      } else if nums1[i as usize] > nums2[j as usize] {
        cur = nums1[i as usize]; i -= 1;
      } else {
        cur = nums2[j as usize]; j -= 1;
      }
      nums1[k as usize] = cur; k -= 1;
    }
  }
}
