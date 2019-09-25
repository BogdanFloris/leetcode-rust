#[allow(dead_code)]
pub fn is_power_of_three(n: i32) -> bool {
    let mut n = n;

    while n > 1 {
        if n % 3 != 0 {
            return false;
        }

        n /= 3;
    }

    n == 1
}

#[cfg(test)]
mod tests {
    use super::is_power_of_three;

    #[test]
    fn basic_1() {
        assert_eq!(is_power_of_three(27), true);
    }

    #[test]
    fn basic_2() {
        assert_eq!(is_power_of_three(0), false);
    }

    #[test]
    fn basic_3() {
        assert_eq!(is_power_of_three(9), true);
    }

    #[test]
    fn basic_4() {
        assert_eq!(is_power_of_three(45), false);
    }
}
