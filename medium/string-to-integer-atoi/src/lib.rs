/// Implement the myAtoi(string s) function, which converts a string to a 32-bit signed integer.
///
/// The algorithm for myAtoi(string s) is as follows:
///
/// 1. Whitespace: Ignore any leading whitespace (" ").
/// 2. Signedness: Determine the sign by checking if the next character is '-' or '+', assuming positivity is neither present.
/// 3. Conversion: Read the integer by skipping leading zeros until a non-digit character is encountered or the end of the string is reached. If no digits were read, then the result is 0.
/// 4. Rounding: If the integer is out of the 32-bit signed integer range [-231, 231 - 1], then round the integer to remain in the range. Specifically, integers less than -231 should be rounded to -231, and integers greater than 231 - 1 should be rounded to 231 - 1.
///
/// Return the integer as the final result.
///
/// # Example
/// ```
/// use string_to_integer_atoi::my_atoi;
///
/// assert_eq!(my_atoi("42".to_string()), 42);
/// assert_eq!(my_atoi("-042".to_string()), -42);
/// assert_eq!(my_atoi("1337c0d3".to_string()), 1337);
/// assert_eq!(my_atoi("words and 987".to_string()), 0);
/// ```
pub fn my_atoi(s: String) -> i32 {
    // 1. Ignore any leading whitespace
    let mut s: &str = s.trim_start();

    // 2. Determine the sign assuming positivity
    let sign = if s.starts_with('-') { -1 } else { 1 };
    s = s.strip_prefix(&['+', '-']).unwrap_or(s);

    // 3. Read integer, skip leading zeros, until non-digit char || end
    s.chars()
        .map(|cha| cha.to_digit(10))
        .take_while(Option::is_some)
        .flatten()
        .fold(0, |out, digit| {
            // 4. Round to the 32-bit signed integer range
            out.saturating_mul(10).saturating_add(digit as i32 * sign)
        })
}
