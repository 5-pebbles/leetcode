/// You are given an integer array score of size n, where score[i] is the score of the ith athlete in a competition. All the scores are guaranteed to be unique.
///
/// The athletes are placed based on their scores, where the 1st place athlete has the highest score, the 2nd place athlete has the 2nd highest score, and so on. The placement of each athlete determines their rank:
///     
/// - The 1st place athletes rank is "Gold Medal".
/// - The 2nd place athletes rank is "Silver Medal".
/// - The 3rd place athletes rank is "Bronze Medal".
/// - For the 4th place to the nth place athlete, their rank is their placement number (i.e., the xth place athletes rank is "x").
///     
/// Return an array answer of size n where answer[i] is the rank of the ith athlete.
///
/// # Example
/// ```
/// use relative_ranks::find_relative_ranks;
///
/// assert_eq!(find_relative_ranks(vec![5, 4, 3, 2, 1]), vec["Gold Medal", "Silver Medal", "Bronze Medal", "4", "5"]);
/// assert_eq!(find_relative_ranks(vec![10, 3, 8, 9, 4]), vec["Gold Medal", "5", "Bronze Medal", "Silver Medal", "4"]);
/// ```
pub fn find_relative_ranks(score: Vec<i32>) -> Vec<String> {
    let mut score: Vec<(usize, i32)> = score.into_iter().enumerate().collect();

    score.sort_unstable_by_key(|v| -v.1);

    let mut answer = vec![String::new(); score.len()];
    for (place, (index, _score)) in score.into_iter().enumerate() {
        let rank = match place {
            0 => "Gold Medal".to_string(),
            1 => "Silver Medal".to_string(),
            2 => "Bronze Medal".to_string(),
            i => (i + 1).to_string(),
        };

        answer[index] = rank;
    }

    answer
}
