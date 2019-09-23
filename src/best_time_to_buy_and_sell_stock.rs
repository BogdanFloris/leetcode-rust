#[allow(dead_code)]
pub fn max_profit(prices: Vec<i32>) -> i32 {
    let mut max_profit = 0;
    let mut min = i32::max_value();

    for price in prices {
        if min > price {
            min = price;
        } else if price - min > max_profit {
            max_profit = price - min;
        }
    }

    max_profit
}

#[cfg(test)]
mod tests {
    use super::max_profit;

    #[test]
    fn basic_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit(prices), 5);
    }

    #[test]
    fn basic_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit(prices), 0);
    }
}
