/// The string "PAYPALISHIRING" is written in a zigzag pattern on a given number of rows like this:
///
/// > P   A   H   N
/// > A P L S I I G
/// > Y   I   R
///
/// And then read line by line: "PAHNAPLSIIGYIR"
///
/// Write the code that will take a string and make this conversion given a number of rows.
/// # Example
/// ```
/// use zigzag_conversion::convert;
///
/// assert_eq!(convert("PAYPALISHIRING".to_string(), 3), "PAHNAPLSIIGYIR");
/// assert_eq!(convert("PAYPALISHIRING".to_string(), 4), "PINALSIGYAHRPI");
/// assert_eq!(convert("A".to_string(), 1), "A");
/// ```
pub fn convert(s: String, num_rows: i32) -> String {
    let vertical = 0..num_rows;
    let diagonal = (1..num_rows - 1).rev();

    let chain_cycle = vertical.chain(diagonal).cycle();

    let mut lines: Vec<_> = chain_cycle.zip(s.chars()).collect();

    lines.sort_by_key(|&(line, _)| line);

    lines.into_iter().map(|(_, cha)| cha).collect()
}
