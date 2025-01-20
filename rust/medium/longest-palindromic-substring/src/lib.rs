/// Given a string s, return the longest palindromic substring in s.
///
/// # Example
/// ```
/// use longest_palindromic_substring::longest_palindrome;
///
/// let result = longest_palindrome("babad".to_string());
/// assert!(result == "bab" || result == "aba");
/// assert_eq!(longest_palindrome("cbbd".to_string()), "bb");
/// ```
pub fn longest_palindrome(s: String) -> String {
    let as_bytes = s.as_bytes();
    let len = as_bytes.len();

    // (left, right)
    let mut longest = (0, 0);

    for index in 0..len * 2 {
        let mut left = index / 2;
        // = left for odd sub-strings
        // = left + 1 for even sub-strings
        let mut right = index / 2 + ((index + 1) % 2);

        // work our way right & left
        while left <= right && right < len {
            if as_bytes[left] != as_bytes[right] {
                break;
            }

            if right - left > longest.1 - longest.0 {
                longest = (left, right);
            }

            // if left wraps it's > right
            left = left.wrapping_sub(1);
            right += 1;
        }
    }

    s[longest.0..=longest.1].to_string()
}
