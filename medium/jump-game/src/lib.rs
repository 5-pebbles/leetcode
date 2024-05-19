/// You are given an integer array `nums`. You are initially positioned at the array's **first index** , and each element in the array represents your maximum jump length at that position.
///
/// Return `true` _if you can reach the last index, or_`false` _otherwise_.
///
/// # Example
/// ```
/// use jump_game::can_jump;
///
/// assert!(can_jump(vec![0]));
/// assert!(!can_jump(vec![0, 1]));
/// assert!(can_jump(vec![2, 3, 1, 1, 4]));
/// assert!(!can_jump(vec![3, 2, 1, 0, 4]));
/// ```
pub fn can_jump(nums: Vec<i32>) -> bool {
    let mut remaning = 1;

    for position in nums.into_iter() {
        if remaning == 0 {
            return false;
        }

        remaning = (remaning - 1).max(position);
    }

    true
}
