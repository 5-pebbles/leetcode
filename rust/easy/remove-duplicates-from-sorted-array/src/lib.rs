/// Given an integer array nums sorted in non-decreasing order, remove the duplicates in-place such that each unique element appears only once. The relative order of the elements should be kept the same. Then return the number of unique elements in nums.
///
/// Consider the number of unique elements of nums to be k, to get accepted, you need to do the following things:
///
/// - Change the array nums such that the first k elements of nums contain the unique elements in the order they were present in nums initially. The remaining elements of nums are not important as well as the size of nums.
/// - Return k.
///
/// # Example
/// ```
/// use remove_duplicates_from_sorted_array::remove_duplicates;
///
/// let mut nums = vec![1, 1, 2];
/// assert_eq!(remove_duplicates(&mut nums), 2);
///
/// let expected = vec![1, 2];
/// for i in 0..expected.len() {
///     assert_eq!(nums[i], expected[i]);
/// }
///
/// let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
/// assert_eq!(remove_duplicates(&mut nums), 5);
///
/// let expected = vec![0,1,2,3,4];
/// for i in 0..expected.len() {
///     assert_eq!(nums[i], expected[i]);
/// }
/// ```
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}
