/// You are given an integer array `prices` where `prices[i]` is the price of a given stock on the `ith` day.
///
/// On each day, you may decide to buy and/or sell the stock. You can only hold **at most one** share of the stock at any time. However, you can buy it then immediately sell it on the **same day**.
///
/// Find and return _the**maximum** profit you can achieve_.
///
/// # Example
/// ```
/// use best_time_to_buy_and_sell_stock_ii::max_profit;
///
/// assert_eq!(max_profit(vec![7, 1, 5, 3, 6, 4]), 7);
/// assert_eq!(max_profit(vec![1, 2, 3, 4, 5]), 4);
/// assert_eq!(max_profit(vec![7, 6, 4, 3, 1]), 0);
/// ```
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut total = 0;
    let mut last = prices[0];

    for price in prices.into_iter().skip(1) {
        total += (price - last).max(0);

        last = price;
    }

    total
}
