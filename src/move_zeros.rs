#[allow(dead_code)]
pub fn move_zeros(nums: &mut Vec<i32>) {
    for i in (0..nums.len()).rev() {
        if nums[i] == 0 {
            nums.remove(i);
            nums.push(0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::move_zeros;

    #[test]
    fn basic() {
        let mut nums = vec![0, 1, 0, 3, 12];
        move_zeros(&mut nums);
        assert_eq!(nums, vec![1, 3, 12, 0, 0]);
    }
}
