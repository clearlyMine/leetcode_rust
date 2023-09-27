use std::collections::HashMap;

//12ms 2.00MB (36.42% 83.24%)
pub fn coin_change_dp(coins: Vec<i32>, amount: i32) -> i32 {
    let coins: Vec<usize> = coins.into_iter().map(|x| x as usize).collect();
    let amount = amount as usize;
    let mut dp: Vec<i32> = vec![i32::MAX; amount + 1];
    dp[0] = 0;
    for i in 1..amount + 1 {
        for c in &coins {
            if i < *c {
                break;
            }
            if dp[i - c] != i32::MAX {
                dp[i] = dp[i].min(1 + dp[i - c]);
            }
        }
    }
    let res = dp[amount];

    return if res == i32::MAX { -1 } else { res };
}

//09ms 2.19MB (65.32% 50.87%)
pub fn coin_change_dp_opt(coins: Vec<i32>, amount: i32) -> i32 {
    let coins: Vec<usize> = coins.into_iter().map(|x| x as usize).collect();
    let amount = amount as usize;
    let mut dp: Vec<Option<i32>> = vec![None; amount + 1];
    dp[0] = Some(0);
    for i in 1..amount + 1 {
        dp[i] = coins
            .iter()
            .filter_map(|j| {
                let j = *j as usize;
                if j <= i {
                    dp[i - j].map(|n| n + 1)
                } else {
                    None
                }
            })
            .min();
    }
    dp[amount].unwrap_or(-1)
}

//TLE
pub fn coin_change_recursive(coins: Vec<i32>, amount: i32) -> i32 {
    fn coin_change_lowest(coins: &Vec<i32>, idx: usize, amount: i32) -> i32 {
        if idx == coins.len() || amount <= 0 {
            return if amount == 0 { 0 } else { i32::MAX - 1 };
        }
        let do_not_take_coin = coin_change_lowest(coins, idx + 1, amount);
        if coins[idx] > amount {
            return do_not_take_coin;
        } else {
            return do_not_take_coin.min(1 + coin_change_lowest(coins, idx, amount - coins[idx]));
        };
    }
    let res = coin_change_lowest(&coins, 0, amount);
    if res == i32::MAX - 1 {
        return -1;
    } else {
        return res;
    }
}

//TLE
pub fn coin_change_memoization(coins: Vec<i32>, amount: i32) -> i32 {
    fn coin_change_lowest(
        coins: &Vec<i32>,
        idx: usize,
        amount: usize,
        dp: HashMap<(usize, usize), i32>,
    ) -> i32 {
        let mut dp = dp;
        if idx == coins.len() || amount <= 0 {
            return if amount == 0 { 0 } else { i32::MAX - 1 };
        }
        if let Some(x) = dp.get(&(idx, amount)) {
            return *x;
        }

        let do_not_take_coin = coin_change_lowest(coins, idx + 1, amount, dp.clone());
        let res = if coins[idx] > amount as i32 {
            do_not_take_coin
        } else {
            do_not_take_coin
                .min(1 + coin_change_lowest(coins, idx, amount - coins[idx] as usize, dp.clone()))
        };
        dp.insert((idx, amount), res);
        res
    }
    let res = coin_change_lowest(&coins, 0, amount as usize, HashMap::new());
    if res == i32::MAX - 1 {
        return -1;
    } else {
        return res;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coin_change_dp() {
        let io = [
            ((vec![1, 2, 5], 11), 3),
            ((vec![2], 3), -1),
            ((vec![1], 0), 0),
            ((vec![186, 419, 83, 408], 6249), 20),
        ];
        io.into_iter().for_each(|((coins, amount), output)| {
            assert_eq!(coin_change_dp(coins, amount), output);
        });
    }

    #[test]
    fn test_coin_change_recursive() {
        let io = [
            ((vec![1, 2, 5], 11), 3),
            ((vec![2], 3), -1),
            ((vec![1], 0), 0),
            ((vec![186, 419, 83, 408], 6249), 20),
        ];
        io.into_iter().for_each(|((coins, amount), output)| {
            assert_eq!(coin_change_recursive(coins, amount), output);
        });
    }

    #[test]
    fn test_coin_change_memoization() {
        let io = [
            ((vec![1, 2, 5], 11), 3),
            ((vec![2], 3), -1),
            ((vec![1], 0), 0),
            ((vec![186, 419, 83, 408], 6249), 20),
        ];
        io.into_iter().for_each(|((coins, amount), output)| {
            assert_eq!(coin_change_memoization(coins, amount), output);
        });
    }

    #[test]
    fn test_coin_change_dp_opt() {
        let io = [
            ((vec![1, 2, 5], 11), 3),
            ((vec![2], 3), -1),
            ((vec![1], 0), 0),
            ((vec![186, 419, 83, 408], 6249), 20),
        ];
        io.into_iter().for_each(|((coins, amount), output)| {
            assert_eq!(coin_change_dp_opt(coins, amount), output);
        });
    }
}
