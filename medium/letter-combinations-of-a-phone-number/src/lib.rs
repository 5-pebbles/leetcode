use std::ops::RangeInclusive;

const MAPPINGS: [RangeInclusive<u8>; 8] = [
    // 2
    (b'a'..=b'c'),
    (b'd'..=b'f'),
    (b'g'..=b'i'),
    (b'j'..=b'l'),
    (b'm'..=b'o'),
    (b'p'..=b's'),
    (b't'..=b'v'),
    // 9
    (b'w'..=b'z'),
];

/// Given a string containing digits from 2-9 inclusive, return all possible letter combinations that the number could represent. Return the answer in any order.
///
/// A mapping of digits to letters is given below:
///
/// - **2:** ```[a, b, c]```
/// - **3:** ```[d, e, f]```
/// - **4:** ```[g, h, i]```
/// - **5:** ```[j, k, l]```
/// - **6:** ```[m, n, o]```
/// - **7:** ```[p, q, r, s]```
/// - **8:** ```[t, u, v]```
/// - **9:** ```[w, x, y, z]```
///
/// # Example
/// ```
/// use letter_combinations_of_a_phone_number::letter_combinations;
///
/// assert_eq!(letter_combinations("".to_string()), Vec::<String>::new());
///
/// let mut result = letter_combinations("23".to_string());
/// result.sort_unstable();
/// assert_eq!(result, vec!["ad","ae","af","bd","be","bf","cd","ce","cf"]);
///
/// let mut result = letter_combinations("2".to_string());
/// result.sort_unstable();
/// assert_eq!(result, vec!["a", "b", "c"]);
/// ```
pub fn letter_combinations(digits: String) -> Vec<String> {
    // avoid returning -> [""]
    if digits.is_empty() {
        return Vec::new();
    }

    // magic (∩ᵔ ᵕ ᵔ )⊃━☆ﾟ.*+.
    digits
        .as_bytes()
        .iter()
        .map(|b| &MAPPINGS[(b - b'2') as usize])
        .fold(vec![String::new()], |combinations: Vec<String>, letters| {
            combinations
                .into_iter()
                .flat_map(|combo| {
                    letters
                        .clone()
                        .map(move |s| format!("{}{}", combo, <char>::from(s)))
                })
                .collect()
        })
}
