#[allow(dead_code)]
pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return nums.len() as i32;
    }

    let mut previous = nums[0];
    let mut current_index = 1;

    for i in 1..nums.len() {
        if nums[i] > previous {
            nums[current_index] = nums[i];
            previous = nums[current_index];
            current_index += 1;
        }
    }

    current_index as i32
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;

    #[test]
    fn basic_1() {
        let mut nums = vec![1, 1, 2];
        assert_eq!(remove_duplicates(&mut nums), 2)
    }

    #[test]
    fn basic_2() {
        let mut nums = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(remove_duplicates(&mut nums), 5);
    }

    #[test]
    fn basic_3() {
        let mut nums = vec![1, 1];
        assert_eq!(remove_duplicates(&mut nums), 1);
    }
}
