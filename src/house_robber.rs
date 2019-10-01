use std::cmp;

#[allow(dead_code)]
pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut dp = vec![0, nums[0]];

    for i in 2..=(nums.len()) {
        dp.push(cmp::max(nums[i - 1] + dp[i - 2], dp[i - 1]));
    }

    dp[nums.len()]
}

#[cfg(test)]
mod tests {
    use super::rob;

    #[test]
    fn basic_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(rob(nums), 4);
    }

    #[test]
    fn basic_2() {
        let nums = vec![2, 7, 9, 3, 1];
        assert_eq!(rob(nums), 12);
    }

    #[test]
    fn basic_3() {
        let nums = vec![2, 1, 1, 2];
        assert_eq!(rob(nums), 4);
    }
}
