#[allow(dead_code)]
pub fn trailing_zeroes(n: i32) -> i32 {
    let mut n = n;
    let mut zeros = 0;

    while n / 5 > 0 {
        n /= 5;
        zeros += n;
    }

    zeros
}

#[cfg(test)]
mod tests {
    use super::trailing_zeroes;

    #[test]
    fn basic_1() {
        assert_eq!(trailing_zeroes(3), 0);
    }

    #[test]
    fn basic_2() {
        assert_eq!(trailing_zeroes(5), 1);
    }
}
