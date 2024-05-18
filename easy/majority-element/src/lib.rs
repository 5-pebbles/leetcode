use std::collections::HashMap;

/// Given an array `nums` of size `n`, return _the majority element_.
///
/// The majority element is the element that appears more than `⌊n / 2⌋` times.
/// You may assume that the majority element always exists in the array.
///
///
/// **Follow-up:** Could you solve the problem in linear time and in `O(1)` space?
///
/// # Example
/// ```
/// use majority_element::majority_element;
///
/// assert_eq!(majority_element(vec![3, 2, 3]), 3);
/// assert_eq!(majority_element(vec![2, 2, 1, 1, 1, 2, 2]), 2);
/// assert_eq!(majority_element(vec![3, 3, 4]), 3);
/// ```
pub fn majority_element(nums: Vec<i32>) -> i32 {
    // yes you can solve with O(1) memory using the boyer-moore majority vote algorithm
    let mut hashmap = HashMap::new();
    let (mut max, mut count): (i32, i32) = (0, 0);

    for num in nums {
        let entry = hashmap.entry(num).and_modify(|v| *v += 1).or_insert(1);

        if *entry > count {
            count = *entry;
            max = num;
        }
    }

    max
}
