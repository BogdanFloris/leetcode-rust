#[allow(dead_code)]
pub fn missing_number(nums: Vec<i32>) -> i32 {
    let mut sum = 0;
    let mut n = 0;
    let mut contains_zero = false;

    for num in nums {
        sum += num;
        if num > n {
            n = num;
        }
        if num == 0 {
            contains_zero = true;
        }
    }

    let sum_all = n * (n + 1) / 2;

    if sum_all == sum && contains_zero {
        n + 1
    } else {
        (n * (n + 1) / 2) - sum
    }
}

#[cfg(test)]
mod tests {
    use super::missing_number;

    #[test]
    fn basic_1() {
        let nums = vec![3, 0, 1];
        assert_eq!(missing_number(nums), 2);
    }

    #[test]
    fn basic_2() {
        let nums = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        assert_eq!(missing_number(nums), 8);
    }
}
