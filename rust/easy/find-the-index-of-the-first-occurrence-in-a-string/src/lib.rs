/// Given two strings `needle` and `haystack`, return the index of the first occurrence of `needle` in `haystack`, or `-1` if `needle` is not part of `haystack`.
///
/// # Example
/// ```
/// use find_the_index_of_the_first_occurrence_in_a_string::str_str;
///
/// assert_eq!(str_str("sadbutsad".to_string(), "sad".to_string()), 0);
/// assert_eq!(str_str("leetcode".to_string(), "leeto".to_string()), -1);
/// ```
pub fn str_str(haystack: String, needle: String) -> i32 {
    let needle = needle.as_bytes();
    haystack
        .as_bytes()
        .windows(needle.len())
        .position(|straw| straw == needle)
        .map(|pos| pos as i32)
        .unwrap_or(-1)
}
