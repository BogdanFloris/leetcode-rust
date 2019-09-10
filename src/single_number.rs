#[allow(dead_code)]
pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut sol = 0;

    for num in nums {
        sol ^= num;
    }

    sol
}

#[cfg(test)]
mod tests {
    use super::single_number;

    #[test]
    fn basic_1() {
        let s = vec![2, 2, 1];
        let sol = single_number(s);
        assert_eq!(sol, 1);
    }

    #[test]
    fn basic_2() {
        let s = vec![4, 1, 2, 1, 2];
        let sol = single_number(s);
        assert_eq!(sol, 4);
    }
}
