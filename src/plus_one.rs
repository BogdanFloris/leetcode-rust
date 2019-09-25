#[allow(dead_code)]
pub fn plus_one(digits: Vec<i32>) -> Vec<i32> {
    let mut digits = digits;
    let mut to_add = 1;

    for i in (0..digits.len()).rev() {
        digits[i] += to_add;
        if digits[i] == 10 {
            digits[i] = 0;
            to_add = 1;
        } else {
            to_add = 0;
        }
    }

    if to_add == 1 {
        digits.insert(0, 1);
    }

    digits
}

#[cfg(test)]
mod tests {
    use super::plus_one;

    #[test]
    fn basic_1() {
        let digits = vec![1, 2, 3];
        assert_eq!(plus_one(digits), vec![1, 2, 4]);
    }

    #[test]
    fn basic_2() {
        let digits = vec![4, 3, 2, 1];
        assert_eq!(plus_one(digits), vec![4, 3, 2, 2]);
    }

    #[test]
    fn basic_3() {
        let digits = vec![1, 9];
        assert_eq!(plus_one(digits), vec![2, 0]);
    }

    #[test]
    fn basic_4() {
        let digits = vec![9, 9];
        assert_eq!(plus_one(digits), vec![1, 0, 0]);
    }
}
