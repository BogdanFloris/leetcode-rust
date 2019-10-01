use std::cmp::max;

#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.is_empty() {
        return 0;
    }

    let mut profit = 0;

    for i in 0..(prices.len() - 1) {
        profit += max(prices[i + 1] - prices[i], 0)
    }

    profit
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn basic_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 7);
    }

    #[test]
    fn basic_2() {
        let prices = vec![1, 2, 3, 4, 5];
        assert_eq!(max_profit(prices), 4);
    }

    #[test]
    fn basic_3() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
