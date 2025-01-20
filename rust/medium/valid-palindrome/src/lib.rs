/// A phrase is a **palindrome** if, after converting all uppercase letters into lowercase letters and removing all non-alphanumeric characters, it reads the same forward and backward. Alphanumeric characters include letters and numbers.
///
/// Given a string `s`, return `true` _if it is a **palindrome**, or_ `false` _otherwise_.
///
/// # Example
/// ```
/// use valid_palindrome::is_palindrome;
///
/// assert_eq!(is_palindrome("A man, a plan, a canal: Panama".to_string()), true);
/// assert_eq!(is_palindrome("race a car".to_string()), false);
/// assert_eq!(is_palindrome("9,8".to_string()), false);
/// assert_eq!(is_palindrome(" ".to_string()), true);
/// ```
pub fn is_palindrome(s: String) -> bool {
    let alphanumeric_bytes: Vec<u8> = s
        .to_lowercase()
        .into_bytes()
        .into_iter()
        .filter(|c| (b'0'..=b'9').contains(c) || (b'a'..=b'z').contains(c))
        .collect();

    (0..(alphanumeric_bytes.len() + 1) / 2)
        .map(|i| (i, alphanumeric_bytes.len() - 1 - i))
        .all(|(left, right)| alphanumeric_bytes[left] == alphanumeric_bytes[right])
}
