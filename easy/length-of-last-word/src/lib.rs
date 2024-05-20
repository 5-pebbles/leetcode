/// Given a string `s` consisting of words and spaces, return _the length of the **last** word in the string._
///
/// A **word** is a maximal substring consisting of non-space characters only.
///
/// # Example
/// ```
/// use length_of_last_word::length_of_last_word;
///
/// assert_eq!(length_of_last_word("Hello World".to_string()), 5);
/// assert_eq!(length_of_last_word("   fly me   to   the moon  ".to_string()), 4);
/// assert_eq!(length_of_last_word("luffy is still joyboy".to_string()), 6);
/// ```
pub fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace()
        .last()
        .map(|s| s.len() as i32)
        .unwrap_or_default()
}
