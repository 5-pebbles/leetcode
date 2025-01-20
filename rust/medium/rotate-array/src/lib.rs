/// Given an integer array `nums`, rotate the array to the right by `k` steps, where `k` is non-negative.
///
/// **Follow up:**
///
///   * Try to come up with as many solutions as you can. There are at least **three** different ways to solve this problem.
///   * Could you do it in-place with `O(1)` extra space?
///
/// # Example
/// ```
/// use rotate_array::rotate;
///
/// let mut input = vec![1, 2, 3, 4, 5, 6, 7];
/// rotate(&mut input, 3);
/// assert_eq!(input, vec![5, 6, 7, 1, 2, 3, 4]);
///
/// let mut input = vec![-1, -100, 3,99];
/// rotate(&mut input, 2);
/// assert_eq!(input, vec![3,99,-1,-100]);
///
/// let mut input = vec![-1];
/// rotate(&mut input, 2);
/// assert_eq!(input, vec![-1]);
/// ```
pub fn rotate(nums: &mut Vec<i32>, k: i32) {
    let len = nums.len();
    // rust is awesome...
    nums.rotate_right(k as usize % len);
}
