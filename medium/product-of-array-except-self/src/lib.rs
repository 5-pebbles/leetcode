/// Given an integer array `nums`, return _an array_ `answer` _such that_ `answer[i]` _is equal to the product of all the elements of_ `nums` _except_ `nums[i]`.
///
/// The product of any prefix or suffix of `nums` is **guaranteed** to fit in a **32-bit** integer.
///
/// You must write an algorithm that runs in `O(n)` time and without using the division operation.
///
/// **Follow up:**  Can you solve the problem in `O(1)` extra space complexity? (The output array **does not** count as extra space for space complexity analysis.)
///
/// # Example
/// ```
/// use product_of_array_except_self::product_except_self;
///
/// assert_eq!(product_except_self(vec![1, 2, 3, 4]), [24, 12, 8, 6]);
/// assert_eq!(product_except_self(vec![-1, 1, 0, -3, 3]), [0, 0, 9, 0, 0]);
/// ```
pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut prefix: i32 = 1;
    let mut suffix: i32 = 1;

    let len = nums.len();
    let mut answer = vec![1; len];

    // think of it like laser eyes \ / idk...
    for index in 0..len {
        let left = index;
        let right = (len - 1) - index;

        answer[left] *= prefix;
        answer[right] *= suffix;

        prefix *= nums[left];
        suffix *= nums[right];
    }

    answer
}
