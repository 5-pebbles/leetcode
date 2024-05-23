/// Given an input string `s`, reverse the order of the **words**.
///
/// A **word** is defined as a sequence of non-space characters. The **words** in `s` will be separated by at least one space.
///
/// Return _a string of the words in reverse order concatenated by a single space._
///
/// **Note** that `s` may contain leading or trailing spaces or multiple spaces between two words. The returned string should only have a single space separating the words. Do not include any extra spaces.
///
/// # Example
/// ```
/// use reverse_words_in_a_string::reverse_words;
///
/// assert_eq!(reverse_words("the sky is blue".to_string()), "blue is sky the");
/// assert_eq!(reverse_words("  hello world  ".to_string()), "world hello");
/// assert_eq!(reverse_words("a good   example".to_string()), "example good a");
/// ```
pub fn reverse_words(s: String) -> String {
    s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
}
