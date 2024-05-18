/// You are given an array `prices` where `prices[i]` is the price of a given stock on the `ith` day.
///
/// You want to maximize your profit by choosing a **single day** to buy one stock and choosing a **different day in the future** to sell that stock.
///
/// Return _the maximum profit you can achieve from this transaction_. If you cannot achieve any profit, return `0`.
///
/// # Example
/// ```
/// use best_time_to_buy_and_sell_stock::max_profit;
///
/// assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
/// assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
/// ```
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut profit = 0;
    let mut min = prices[0];

    for price in prices.into_iter().skip(1) {
        profit = profit.max(price - min);
        min = min.min(price);
    }

    profit
}
