#[allow(dead_code)]
pub fn majority_element(nums: Vec<i32>) -> i32 {
    let mut copy = nums.to_vec();
    copy.sort();
    copy[copy.len() / 2]
}

#[cfg(test)]
mod tests {
    use super::majority_element;

    #[test]
    fn basic_1() {
        let nums = vec![3, 2, 3];
        assert_eq!(majority_element(nums), 3);
    }

    #[test]
    fn basic_2() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        assert_eq!(majority_element(nums), 2);
    }
}
