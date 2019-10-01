#[allow(dead_code)]
pub fn find_duplicate(nums: Vec<i32>) -> i32 {
    let mut slow = nums[0];
    let mut fast: i32 = nums[nums[0] as usize];

    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[nums[fast as usize] as usize];
    }

    slow = 0;
    while slow != fast {
        slow = nums[slow as usize];
        fast = nums[fast as usize];
    }

    slow
}

#[cfg(test)]
mod tests {
    use super::find_duplicate;

    #[test]
    fn basic_1() {
        let nums = vec![1, 3, 4, 2, 2];
        assert_eq!(find_duplicate(nums), 2);
    }

    #[test]
    fn basic_2() {
        let nums = vec![3, 1, 3, 4, 2];
        assert_eq!(find_duplicate(nums), 3);
    }
}
