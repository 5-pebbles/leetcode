use std::collections::HashMap;

/// Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it can trap after raining.
///
/// # Example
/// ```
/// use trapping_rain_water::trap;
///
/// assert_eq!(trap(vec![1,0,1]), 1);
/// assert_eq!(trap(vec![0,1,0,2,1,0,1,3,2,1,2,1]), 6);
/// assert_eq!(trap(vec![4,2,0,3,2,5]), 9);
/// ```
pub fn trap(height: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut max = 0;

    let (mut left, mut right) = (0, height.len() - 1);

    while left < right {
        let (lefth, righth) = (height[left], height[right]);

        max = max.max(lefth.min(righth));

        if lefth >= righth {
            total += 0.max(max - righth);
            right -= 1;
        } else {
            total += 0.max(max - lefth);
            left += 1;
        }
    }

    total
}
