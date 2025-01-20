/// You are given two integer arrays `nums1` and `nums2`, sorted in **non-decreasing order** , and two integers `m` and `n`, representing the number of elements in `nums1` and `nums2` respectively.
///
/// **Merge** `nums1` and `nums2` into a single array sorted in **non-decreasing
/// order**.
///
/// The final sorted array should not be returned by the function, but instead be _stored inside the array_`nums1`. To accommodate this, `nums1` has a length of `m + n`, where the first `m` elements denote the elements that should be merged, and the last `n` elements are set to `0` and should be ignored. `nums2` has a length of `n`.
///
/// # Example
/// ```
/// use merge_sorted_array::merge;
///
/// let (mut result, mut input) = (vec![1,2,3,0,0,0], vec![2,5,6]);
/// merge(&mut result, 3, &mut input, 3);
/// assert_eq!(result, [1,2,2,3,5,6]);
///
/// let (mut result, mut input) = (vec![1], vec![]);
/// merge(&mut result, 1, &mut input, 0);
/// assert_eq!(result, [1]);
///
/// let (mut result, mut input) = (vec![], vec![1]);
/// merge(&mut result, 0, &mut input, 1);
/// assert_eq!(result, [1]);
/// ```
pub fn merge(left: &mut Vec<i32>, left_len: i32, right: &mut Vec<i32>, _right_len: i32) {
    left.truncate(left_len as usize);
    left.append(right);
    left.sort_unstable();
}
