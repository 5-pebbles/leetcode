/// Given an array of integers `citations` where `citations[i]` is the number of citations a researcher received for their `ith` paper, return _the researcher 's h-index_.
///
/// According to the [definition of h-index on Wikipedia](https://en.wikipedia.org/wiki/H-index): The h-index is defined as the maximum value of `h` such that the given researcher has published at least `h` papers that have each been cited at least `h` times.
///
/// # Example
/// ```
/// use h_index::h_index;
///
/// assert_eq!(h_index(vec![3, 0, 6, 1, 5]), 3);
/// assert_eq!(h_index(vec![1, 3, 1]), 1);
/// assert_eq!(h_index(vec![4, 4, 0, 0]), 2);
/// assert_eq!(h_index(vec![100]), 1);
/// ```
pub fn h_index(mut citations: Vec<i32>) -> i32 {
    citations.sort_unstable();

    for (index, citation) in citations.iter().rev().enumerate() {
        if index >= *citation as usize {
            return index as i32;
        }
    }

    citations.len() as i32
}
