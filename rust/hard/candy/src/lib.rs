/// There are `n` children standing in a line. Each child is assigned a rating value given in the integer array `ratings`.
///
/// You are giving candies to these children subjected to the following requirements:
///
///  - Each child must have at least one candy.
///  - Children with a higher rating get more candies than their neighbors.
///
/// Return _the minimum number of candies you need to have to distribute the candies to the children_.
///
/// # Example
/// ```
/// use candy::candy;
///
/// assert_eq!(candy(vec![1, 2, 2]), 4);
/// assert_eq!(candy(vec![1, 0, 2]), 5);
/// ```
pub fn candy(ratings: Vec<i32>) -> i32 {
    let len = ratings.len();
    let mut candy: Vec<i32> = vec![1; len];

    // lazer eyes \ /
    // ->
    for index in 0..len - 1 {
        let left = index;
        let right = index + 1;

        if ratings[left] < ratings[right] {
            candy[right] = candy[left] + 1;
        }
    }

    // <-
    for index in (1..len).rev() {
        let left = index - 1;
        let right = index;

        if ratings[left] > ratings[right] {
            candy[left] = candy[left].max(candy[right] + 1);
        }
    }

    candy.into_iter().sum()
}
