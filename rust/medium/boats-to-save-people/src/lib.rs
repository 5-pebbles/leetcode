/// You are given an array people where people[i] is the weight of the ith person, and an infinite number of boats where each boat can carry a maximum weight of limit. Each boat carries at most two people at the same time, provided the sum of the weight of those people is at most limit.
///
/// Return the minimum number of boats to carry every given person.
/// # Example
/// ```
/// use boats_to_save_people::num_rescue_boats;
///
/// assert_eq!(num_rescue_boats(vec![1, 2], 3), 1);
/// assert_eq!(num_rescue_boats(vec![3, 2, 2, 1], 3), 3);
/// assert_eq!(num_rescue_boats(vec![3, 5, 3, 4], 5), 4);
/// ```
pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
    people.sort_unstable();
    let (mut left, mut right) = (0, people.len() - 1);
    let mut boats = 0;

    while left < right {
        if people[left] + people[right] <= limit {
            left += 1;
        }

        right -= 1;
        boats += 1;
    }

    if left == right {
        return boats + 1;
    }

    boats
}
