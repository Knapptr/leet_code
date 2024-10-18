use crate::solution::Solution;
/*
You are given an array prices where prices[i] is the price of a given stock on the ith day.

You want to maximize your profit by choosing a single day to buy one stock and choosing a different day in the future to sell that stock.

Return the maximum profit you can achieve from this transaction. If you cannot achieve any profit, return 0.
*/

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = 0;
        let mut sell = 0;
        let mut max = 0;
        while sell < prices.len() - 1 {
            sell += 1;
            let buy_value = prices[buy];
            let sell_value = prices[sell];
            let total_profit = sell_value - buy_value;
            max = max.max(total_profit);

            if sell_value < buy_value {
                buy = sell;
            }
        }
        return max;
    }
}

#[cfg(test)]
#[test]
fn example_one() {
    let prices = vec![7, 1, 2, 3, 6, 2];
    let expected = 5;
    let solved = Solution::max_profit(prices);
    assert_eq!(solved, expected);
}
#[test]
fn example_two() {
    let prices = vec![7, 6, 4, 3, 1];
    let expected = 0;
    let solved = Solution::max_profit(prices);
    assert_eq!(solved, expected);
}
