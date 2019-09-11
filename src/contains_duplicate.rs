use std::collections::HashMap;

#[allow(dead_code)]
pub fn contains_duplicate(nums: Vec<i32>) -> bool {
    let mut map: HashMap<i32, bool> = HashMap::new();

    for num in nums {
        if map.contains_key(&num) {
            return true;
        }
        map.insert(num, true);
    }

    false
}

#[cfg(test)]
mod tests {
    use super::contains_duplicate;

    #[test]
    fn basic_1() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(contains_duplicate(nums), true);
    }

    #[test]
    fn basic_2() {
        let nums = vec![1, 2, 3, 4];
        assert_eq!(contains_duplicate(nums), false);
    }

    #[test]
    fn basic_3() {
        let nums = vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2];
        assert_eq!(contains_duplicate(nums), true);
    }
}
