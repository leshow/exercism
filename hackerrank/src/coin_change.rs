// 322. Coin Change
// Medium
// You are given coins of different denominations and a total amount of money
// amount. Write a function to compute the fewest number of coins that you need
// to make up that amount. If that amount of money cannot be made up by any
// combination of the coins, return -1. Example 1:
// Input: coins = [1, 2, 5], amount = 11
// Output: 3
// Explanation: 11 = 5 + 5 + 1
// Example 2:
// Input: coins = [2], amount = 3
// Output: -1

use std::cmp;

fn coin_change(coins: Vec<i32>, amount: i32) -> i32 {
    dp(&coins, amount, coins.len())
}

fn dp(coins: &[i32], amount: i32, i: usize) -> i32 {
    if i == 0 {
        return 0;
    }
    if amount <= 0 {
        return 0;
    }
    if coins[i - 1] > amount {
        dp(coins, amount, i - 1)
    } else {
        cmp::min(
            1 + dp(coins, amount - coins[i - 1], i - 1),
            dp(coins, amount, i - 1),
        )
    }
}
