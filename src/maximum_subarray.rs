use std::cmp;

#[allow(dead_code)]
pub fn max_sub_array(nums: Vec<i32>) -> i32 {
    if nums.len() == 1 {
        return nums[0];
    }

    let mut res = nums[0];
    let mut nums = nums;

    for i in 1..nums.len() {
        nums[i] = cmp::max(nums[i - 1] + nums[i], nums[i]);
        if res < nums[i] {
            res = nums[i];
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::max_sub_array;

    #[test]
    fn basic() {
        let nums = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        assert_eq!(max_sub_array(nums), 6);
    }
}
