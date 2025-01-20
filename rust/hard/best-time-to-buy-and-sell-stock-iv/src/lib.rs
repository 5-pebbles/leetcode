/// You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `ith` day, and an integer `k`.
///
/// Find the maximum profit you can achieve. You may complete at most `k` transactions: i.e. you may buy at most `k` times and sell at most `k` times.
///
/// **Note:** You may not engage in multiple transactions simultaneously (i.e., you must sell the stock before you buy again).
///
/// # Example
/// ```
/// use best_time_to_buy_and_sell_stock_iv::max_profit;
///
/// assert_eq!(max_profit(2, vec![2, 4, 1]), 2);
/// assert_eq!(max_profit(2, vec![3, 2, 6, 5, 0, 3]), 7);
/// assert_eq!(max_profit(2, vec![1, 2, 4, 2, 5, 7, 2, 4, 9, 0]), 13);
/// ```
pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
    let transaction_limit = k as usize;

    // `cache[day].0` represents the maximum profit made by holding the stock on `day`.
    // `cache[day].1` represents the maximum profit made by selling the stock on or before `day`.
    let mut cache = vec![(0, 0); prices.len() + 1];

    // `last_cache` stores the maximum profit with the last transaction limit.
    let mut last_cache = cache.clone();

    for _ in 0..transaction_limit {
        for (day, price) in prices.iter().enumerate().rev() {
            // Update `cache[day].0` with the maximum of:
            // 1. Buying the stock today (current price subtracted from the maximum profit made by selling before).
            let buy = cache[day + 1].1 - price;
            // 2. Holding the stock from the previous day.
            let hold = cache[day + 1].0;
            cache[day].0 = buy.max(hold);

            // Update `cache[day].1` with the maximum of:
            // 1. Selling the stock today (maximum profit made by holding the stock until the previous day plus the current price).
            let today = last_cache[day + 1].0 + price;
            // 2. Selling before
            let before = cache[day + 1].1;
            cache[day].1 = today.max(before);
        }

        // Swap the pointers without moving the underlying memory.
        std::mem::swap(&mut last_cache, &mut cache);
    }

    // Return the maximum profit made by selling the stock on or before the last day.
    last_cache[0].0
}
