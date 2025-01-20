/// You are given a **0-indexed** array of integers `nums` of length `n`. You are initially positioned at `nums[0]`.
///
/// Each element `nums[i]` represents the maximum length of a forward jump from index `i`. In other words, if you are at `nums[i]`, you can jump to any `nums[i + j]` where:
///
///  - `0 <= j <= nums[i]` and
///  - `i + j < n`
///
/// Return _the minimum number of jumps to reach_`nums[n - 1]`.
///
/// > Note: you can always reach `nums[n - 1]`.
///
/// # Example
/// ```
/// use jump_game_ii::jump;
///
/// assert_eq!(jump(vec![2, 1]), 1);
/// assert_eq!(jump(vec![2, 3, 1, 1, 4]), 2);
/// assert_eq!(jump(vec![2, 3, 0, 1, 4]), 2);
/// ```
pub fn jump(nums: Vec<i32>) -> i32 {
    let (mut remaining, mut max) = (1, 0);
    let mut total = 0;

    for jump in nums.into_iter() {
        if remaining == 0 {
            remaining = max;
            total += 1;
        }

        max = (max - 1).max(jump);
        remaining -= 1;
    }

    total
}
