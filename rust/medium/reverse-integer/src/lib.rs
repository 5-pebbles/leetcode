/// Given a signed 32-bit integer x, return x with its digits reversed. If reversing x causes the value to go outside the signed 32-bit integer range [-231, 231 - 1], then return 0.
///
/// Assume the environment does not allow you to store 64-bit integers (signed or unsigned).
/// # Example
/// ```
/// use reverse_integer::reverse;
///
/// assert_eq!(reverse(123), 321);
/// assert_eq!(reverse(-123), -321);
/// assert_eq!(reverse(120), 21);
/// assert_eq!(reverse(1534236469), 0);
/// ```
pub fn reverse(x: i32) -> i32 {
    /// A more idiomatic function signature
    fn internal(mut x: i32) -> Option<i32> {
        let mut reversed: i32 = 0;

        while x != 0 {
            let last_digit = x % 10;

            reversed = reversed
                .checked_mul(10)
                .and_then(|r| r.checked_add(last_digit))?;

            x /= 10;
        }

        Some(reversed)
    }

    internal(x).unwrap_or_default()
}
