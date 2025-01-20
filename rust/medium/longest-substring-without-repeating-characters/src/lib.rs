use std::collections::VecDeque;

/// Given a string s, find the length of the longest substring without repeating characters.
///
/// # Example
/// ```
/// use longest_substring_without_repeating_characters::length_of_longest_substring;
///
/// assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
/// assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
/// assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
/// assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
/// ```
pub fn length_of_longest_substring(s: String) -> i32 {
    let mut longest: i32 = 0;
    let mut sel_len: i32 = 0;

    let mut selection = VecDeque::new();

    for cha in s.chars() {
        if selection.contains(&cha) {
            // update the longest selection
            longest = longest.max(sel_len);

            // remove all chars up to the dup
            while let Some(sel_end) = selection.pop_back() {
                sel_len -= 1;
                if cha == sel_end {
                    break;
                }
            }
        }

        selection.push_front(cha);
        sel_len += 1;
    }
    // update the longest selection once more
    longest = longest.max(sel_len);

    longest
}
