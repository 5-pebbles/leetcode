/// Given an integer array `nums` sorted in **non-decreasing order** , remove some duplicates [**in-place**](https://en.wikipedia.org/wiki/In-place_algorithm) such that each unique element appears **at most twice**. The **relative order** of the elements should be kept the **same**.
///
/// - Since it is impossible to change the length of the array in some languages, you must instead have the result be placed in the **first part** of the array `nums`. More formally, if there are `k` elements after removing the duplicates, then the first `k` elements of `nums` should hold the final result. It does not matter what you leave beyond the first `k` elements.
///
/// - Return `k` _after placing the final result in the first_`k` _slots of_`nums`.
///
/// Do **not** allocate extra space for another array. You must do this by **modifying the input array[in-place](https://en.wikipedia.org/wiki/In-place_algorithm)** with O(1) extra memory.
///
/// # Example
/// ```
/// use remove_duplicates_from_sorted_array_ii::remove_duplicates;
///
/// let mut input = vec![1, 1 ,1, 2, 2, 3];
/// assert_eq!(remove_duplicates(&mut input), 5);
/// assert_eq!(input, [1, 1, 2, 2, 3]);
///
/// let mut input = vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
/// assert_eq!(remove_duplicates(&mut input), 7);
/// assert_eq!(input, [0, 0, 1, 1, 2, 3, 3]);
/// ```
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let (mut last, mut count) = (0, 0);

    nums.retain(|x| {
        let retain = &last != x || count < 2;

        if x != &last {
            count = 1;
        } else {
            count += 1;
        }

        last = *x;
        retain
    });

    nums.len() as i32
}
