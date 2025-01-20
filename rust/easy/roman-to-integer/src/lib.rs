/// Roman numerals are represented by seven different symbols: `I`, `V`, `X`, `L`, `C`, `D` and `M`.
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
/// For example, `2` is written as `II` in Roman numeral, just two ones added together. `12` is written as `XII`, which is simply `X + II`. The number `27` is written as `XXVII`, which is `XX + V + II`.
///
/// Roman numerals are usually written largest to smallest from left to right. However, the numeral for four is not `IIII`. Instead, the number four is written as `IV`. Because the one is before the five we subtract it making four. The same principle applies to the number nine, which is written as `IX`.
/// There are six instances where subtraction is used:
///
///  - `I` can be placed before `V` (5) and `X` (10) to make 4 and 9.
///  - `X` can be placed before `L` (50) and `C` (100) to make 40 and 90.
///  - `C` can be placed before `D` (500) and `M` (1000) to make 400 and 900.
///
/// # Example
/// ```
/// use roman_to_integer::roman_to_int;
///
/// assert_eq!(roman_to_int("III".to_string()), 3);
/// assert_eq!(roman_to_int("LVIII".to_string()), 58);
/// assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
/// ```
pub fn roman_to_int(s: String) -> i32 {
    fn numeral_to_i32(numeral: &u8) -> i32 {
        match *numeral {
            b'I' => 1,
            b'V' => 5,
            b'X' => 10,
            b'L' => 50,
            b'C' => 100,
            b'D' => 500,
            b'M' => 1000,
            _ => unreachable!(),
        }
    }

    s.as_bytes()
        .into_iter()
        .map(numeral_to_i32)
        .rev()
        .fold((0, 0), |(mut total, last), this| {
            if this < last {
                total -= this;
            } else {
                total += this;
            }
            (total, this)
        })
        .0
}
