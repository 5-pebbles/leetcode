#[rustfmt::skip]
const ROMAN_NUMERALS: [(&str, i32); 13] = [
    ("I",  1),
    ("IV", 4),
    ("V",  5),
    ("IX", 9),
    ("X",  10),
    ("XL", 40),
    ("L",  50),
    ("XC", 90),
    ("C",  100),
    ("CD", 400),
    ("D",  500),
    ("CM", 900),
    ("M",  1000),
];

/// Seven different symbols represent Roman numerals with the following values:
///
/// | Symbol | Value |
/// |--------|-------|
/// |   I    |   1   |
/// |   V    |   5   |
/// |   X    |  10   |
/// |   L    |  50   |
/// |   C    | 100   |
/// |   D    | 500   |
/// |   M    | 1000  |
///   
/// Roman numerals are formed by appending the conversions of decimal place values
/// from highest to lowest. Converting a decimal place value into a Roman numeral
/// has the following rules:
///
/// - If the value does not start with 4 or 9, select the symbol of the maximal value that can be subtracted from the input, append that symbol to the result, subtract its value, and convert the remainder to a Roman numeral.
/// - If the value starts with 4 or 9 use the **subtractive form**  representing one symbol subtracted from the following symbol, for example, 4 is 1 (`I`) less than 5 (`V`): `IV` and 9 is 1 (`I`) less than 10 (`X`): `IX`. Only the following subtractive forms are used: 4 (`IV`), 9 (`IX`), 40 (`XL`), 90 (`XC`), 400 (`CD`) and 900 (`CM`).
/// - Only powers of 10 (`I`, `X`, `C`, `M`) can be appended consecutively at most 3 times to represent multiples of 10. You cannot append 5 (`V`), 50 (`L`), or 500 (`D`) multiple times. If you need to append a symbol 4 times use the **subtractive form**.
///
/// # Example
/// ```
/// use integer_to_roman::int_to_roman;
///
/// assert_eq!(int_to_roman(3749), "MMMDCCXLIX");
/// assert_eq!(int_to_roman(58), "LVIII");
/// assert_eq!(int_to_roman(1994), "MCMXCIV");
/// ```
pub fn int_to_roman(mut num: i32) -> String {
    let mut roman = String::new();

    for (numeral, value) in ROMAN_NUMERALS.iter().rev() {
        while num >= *value {
            roman.push_str(numeral);
            num -= value;
        }
    }

    roman
}
