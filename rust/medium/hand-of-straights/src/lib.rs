use std::collections::{btree_map::Entry, BTreeMap};

/// Alice has some number of cards and she wants to rearrange the cards into groups so that each group is of size `groupSize`, and consists of `groupSize` consecutive cards.
///
/// Given an integer array `hand` where `hand[i]` is the value written on the `ith` card and an integer `groupSize`, return `true` if she can rearrange the cards, or `false` otherwise.
///
/// # Example
/// ```
/// use hand_of_straights::is_n_straight_hand;
///
/// assert_eq!(is_n_straight_hand(vec![1, 2, 3, 6, 2, 3, 4, 7, 8], 3), true);
/// assert_eq!(is_n_straight_hand(vec![1, 2, 3, 4, 5], 4), false);
/// assert_eq!(is_n_straight_hand(vec![1, 2, 3], 1), true);
/// ```
pub fn is_n_straight_hand(hand: Vec<i32>, group_size: i32) -> bool {
    if hand.len() % group_size as usize != 0 {
        return false;
    }
    let mut hand: BTreeMap<i32, i32> = hand.into_iter().fold(BTreeMap::new(), |mut hand, x| {
        hand.entry(x).and_modify(|e| *e += 1).or_insert(1);
        hand
    });

    while !hand.is_empty() {
        let key = *hand.keys().next().unwrap();

        for index in 0..group_size {
            match hand.entry(key + index).and_modify(|e| *e -= 1) {
                Entry::Occupied(entry) => {
                    if *entry.get() == 0 {
                        entry.remove();
                    }
                }
                Entry::Vacant(_) => return false,
            }
        }
    }
    true
}
