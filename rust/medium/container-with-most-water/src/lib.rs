/// You are given an integer array height of length n. There are n vertical lines drawn such that the two endpoints of the ith line are (i, 0) and (i, height[i]).
///
/// Find two lines that together with the x-axis form a container, such that the container contains the most water.
///
/// Return the maximum amount of water a container can store.
/// ```
/// use container_with_most_water::max_area;
///
/// assert_eq!(max_area(vec![1,1]), 1);
/// assert_eq!(max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]), 49);
/// ```
pub fn max_area(height: Vec<i32>) -> i32 {
    let (mut left, mut right) = (0, height.len() - 1);

    let mut max_volume = 0;
    while left != right {
        max_volume = max_volume.max((right - left) as i32 * height[left].min(height[right]));

        if height[left] < height[right] {
            left += 1;
        } else {
            right -= 1;
        }
    }
    max_volume
}
