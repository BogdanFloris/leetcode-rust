use std::collections::HashMap;

#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (index, num) in nums.iter().enumerate() {
        let diff = target - *num;
        if map.contains_key(&diff) {
            return vec![*map.get(&diff).unwrap() as i32, index as i32];
        } else {
            map.insert(*num, index);
        }
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn basic() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
