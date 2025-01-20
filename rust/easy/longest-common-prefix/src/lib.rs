/// Write a function to find the longest common prefix string amongst an array of strings.
///
/// If there is no common prefix, return an empty string `""`.
///
/// # Example
/// ```
/// use longest_common_prefix::longest_common_prefix;
///
/// assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl");
/// assert_eq!(longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "");
/// ```
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    strs.into_iter()
        .reduce(|last, this| -> String {
            last.chars()
                .zip(this.chars())
                .take_while(|(a, b)| a == b)
                .map(|(a, _)| a)
                .collect()
        })
        .unwrap_or_default()
}
