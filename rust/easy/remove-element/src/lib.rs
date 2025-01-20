/// Given an integer array `nums` and an integer `val`, remove all occurrences of `val` in `nums` [**in-place**](https://en.wikipedia.org/wiki/In-place_algorithm). The order of the elements may be changed. Then return _the number of elements in_`nums` _which are not equal to_`val`.
///
/// Consider the number of elements in `nums` which are not equal to `val` be `k`, to get accepted, you need to do the following things:
///
/// - Change the array `nums` such that the first `k` elements of `nums` contain the elements which are not equal to `val`. The remaining elements of `nums` are not important as well as the size of `nums`.
/// - Return `k`.
///
/// # Example
/// ```
/// use remove_element::remove_element;
///
/// let mut input = vec![3, 2, 2, 3];
/// assert_eq!(remove_element(&mut input, 3), 2);
/// input.sort();
/// assert_eq!(input, [2, 2]);
///
/// let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
/// assert_eq!(remove_element(&mut input, 2), 5);
/// input.sort();
/// assert_eq!(input, [0, 0, 1, 3, 4]);
/// ```
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    nums.retain(|x| x != &val);
    nums.len() as i32
}
