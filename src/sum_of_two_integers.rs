#[allow(dead_code)]
pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut a = i64::from(a);
    let mut b = i64::from(b);
    let mask: i64 = 0xffff_ffff;

    while b & mask != 0 {
        let carry = a & b;
        a ^= b;
        b = carry << 1;
    }

    if b > mask {
        return (a & mask) as i32;
    }

    a as i32
}

#[cfg(test)]
mod tests {
    use super::get_sum;

    #[test]
    fn basic_1() {
        let a = 1;
        let b = 2;
        assert_eq!(get_sum(a, b), 3);
    }

    #[test]
    fn basic_2() {
        let a = -2;
        let b = 3;
        assert_eq!(get_sum(a, b), 1);
    }

    #[test]
    fn basic_3() {
        let a = -2;
        let b = 2;
        assert_eq!(get_sum(a, b), 0);
    }
}
